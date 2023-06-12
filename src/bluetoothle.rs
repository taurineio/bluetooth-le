use btleplug::{
    api::{BDAddr, Central, Manager as _, Peripheral, ScanFilter},
    platform::{Adapter, Manager},
};
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
    #[error("No Bluetooth adapter found")]
    NoAdapter,
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

async fn get_adapters() -> Result<Vec<Adapter>, BluetoothLEError> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await.unwrap_or(Vec::new());
    if adapter_list.is_empty() {
        Err(BluetoothLEError::NoAdapter)
    } else {
        Ok(adapter_list)
    }
}

pub(crate) async fn start_scan(
    tx: async_runtime::Sender<BluetoothLEMessage>,
    timeout: Option<u64>,
) -> Result<(), BluetoothLEError> {
    let adapter_list = get_adapters().await?;

    for adapter in adapter_list.iter() {
        adapter.start_scan(ScanFilter::default()).await?;
    }

    let timeout: u128 = timeout.unwrap_or(10000).into();
    let tx_end: async_runtime::Sender<BluetoothLEMessage> = tx.clone();
    async_runtime::spawn(async move {
        let start = get_current_millis();
        let mut discovered: Vec<BDAddr> = Vec::new();
        while get_current_millis() - start < timeout {
            for adapter in adapter_list.iter() {
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
                                let _ =
                                    ctx.send(BluetoothLEMessage::DeviceDiscovered(device)).await;
                            });
                        }
                    }
                }
            }
            sleep(Duration::from_millis(100)).await;
        }
        let _ = tx_end.send(BluetoothLEMessage::StopScan).await;
    });
    Ok(())
}

pub(crate) async fn stop_scan() -> Result<(), BluetoothLEError> {
    let adapter_list = get_adapters().await?;

    for adapter in adapter_list.iter() {
        adapter.stop_scan().await?;
    }

    Ok(())
}
