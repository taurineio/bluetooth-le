use btleplug::{
    api::{BDAddr, Central, Manager as _, Peripheral, ScanFilter},
    platform::{Adapter, Manager},
};
use once_cell::sync::OnceCell;
use serde::{ser::Serializer, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::async_runtime;
use tokio::time::{sleep, Duration};

use crate::models::*;

pub enum BluetoothLEMessage {
    DeviceDiscovered(BluetoothLEDevice),
    StopScan,
}

#[derive(Debug, thiserror::Error)]
pub enum BluetoothLEError {
    #[error("Scan in progress")]
    ScanInProgress,
    #[error("Manager not initialized")]
    NoManager,
    #[error("No Bluetooth adapter found")]
    NoAdapter,
    #[error("Peripheral not found")]
    NoPeripheral,
    #[error("Already connected")]
    AlreadyConnected,
    #[error(transparent)]
    BtlePlug(#[from] btleplug::Error),
}

impl Serialize for BluetoothLEError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

fn get_current_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

static MANAGER: OnceCell<Manager> = OnceCell::new();
static ADAPTER: OnceCell<Adapter> = OnceCell::new();

async fn get_manager() -> Result<&'static Manager, BluetoothLEError> {
    if MANAGER.get().is_none() {
        let manager = Manager::new().await?;
        MANAGER.set(manager).unwrap();
    }
    MANAGER.get().ok_or(BluetoothLEError::NoManager)
}

async fn get_adapters() -> Result<Vec<Adapter>, BluetoothLEError> {
    let manager = get_manager().await?;
    let adapters = manager.adapters().await.unwrap_or(Vec::new());
    if adapters.is_empty() {
        Err(BluetoothLEError::NoAdapter)
    } else {
        Ok(adapters)
    }
}

async fn get_adapter() -> Result<&'static Adapter, BluetoothLEError> {
    if ADAPTER.get().is_none() {
        let manager = get_manager().await?;
        let adapters = manager.adapters().await.unwrap_or(Vec::new());
        if adapters.is_empty() {
            return Err(BluetoothLEError::NoAdapter);
        } else {
            let adapter = adapters[0].clone();
            ADAPTER.set(adapter).unwrap();
        }
    }
    ADAPTER.get().ok_or(BluetoothLEError::NoAdapter)
}

async fn get_periperal(address: String) -> Result<impl Peripheral, BluetoothLEError> {
    let adapter = get_adapter().await?;
    for peripheral in adapter.peripherals().await? {
        if peripheral.address().to_string() == address {
            return Ok(peripheral);
        }
    }
    Err(BluetoothLEError::NoPeripheral)
}

pub(crate) async fn start_scan(
    tx: async_runtime::Sender<BluetoothLEMessage>,
    request: Option<ScanBleRequest>,
) -> Result<(), BluetoothLEError> {
    // let adapters = get_adapters().await?;
    let ScanBleRequest { filter, timeout } = request.unwrap_or_default();
    let filter = filter.unwrap_or(ScanFilter::default());

    let adapter = get_adapter().await?;
    adapter.start_scan(filter).await?;
    // for adapter in adapters.iter() {
    //     adapter.start_scan(filter.clone()).await?;
    // }

    let timeout: u128 = timeout.unwrap_or(10000).into();
    let tx_end: async_runtime::Sender<BluetoothLEMessage> = tx.clone();
    async_runtime::spawn(async move {
        let start = get_current_millis();
        let mut discovered: Vec<BDAddr> = Vec::new();
        let adapter = get_adapter().await.unwrap();
        while get_current_millis() - start < timeout {
            // for adapter in adapters.iter() {
            let peripherals = adapter.peripherals().await.unwrap_or_default();
            for peripheral in peripherals.iter() {
                let props = peripheral.properties().await;
                if let Ok(Some(props)) = props {
                    let address = props.address.clone();
                    if !discovered.contains(&address) {
                        discovered.push(address);
                        let ctx = tx.clone();
                        let device = BluetoothLEDevice {
                            address: address.to_string(),
                            name: props.local_name,
                            uuids: props
                                .service_data
                                .into_iter()
                                .map(|s| s.0.to_string())
                                .collect(),
                        };
                        async_runtime::spawn(async move {
                            let _ = ctx.send(BluetoothLEMessage::DeviceDiscovered(device)).await;
                        });
                    }
                }
            }
            // }
            sleep(Duration::from_millis(100)).await;
        }
        let _ = tx_end.send(BluetoothLEMessage::StopScan).await;
    });
    Ok(())
}

pub(crate) async fn stop_scan() -> Result<(), BluetoothLEError> {
    // let adapters = get_adapters().await?;

    // for adapter in adapters.iter() {
    //     adapter.stop_scan().await?;
    // }
    let adapter = get_adapter().await?;
    adapter.stop_scan().await?;

    Ok(())
}

pub(crate) async fn connect(request: ConnectRequest) -> Result<(), BluetoothLEError> {
    let peripheral = get_periperal(request.address).await?;
    peripheral.connect().await?;
    peripheral.discover_services().await?;
    Ok(())
}

pub(crate) async fn get_services(
    request: GetServicesRequest,
) -> Result<Vec<Service>, BluetoothLEError> {
    let peripheral = get_periperal(request.address).await?;
    Ok(peripheral
        .services()
        .into_iter()
        .map(|service| service.into())
        .collect())
}
