use tauri::{command, AppHandle, Runtime, State, Window};

use crate::{
    bluetoothle, models::*, BluetoothLEChannel, BluetoothLEScanning, ConnectedDevices, Error,
    Result, TaurineBluetoothLEExt,
};

#[command]
pub(crate) async fn init<R: Runtime>(app: AppHandle<R>, payload: InitRequest) -> Result<()> {
    #[cfg(target_os = "android")]
    {
        let android = payload.android.unwrap_or_default();
        app.taurine_bluetooth_le().init_ble(android)
    }
    #[cfg(not(target_os = "android"))]
    app.taurine_bluetooth_le().init_ble()
}

#[command]
pub(crate) async fn start_scan<R: Runtime>(
    payload: Option<ScanBleRequest>,
    _app: AppHandle<R>,
    _window: Window<R>,
    channel_state: State<'_, BluetoothLEChannel>,
    scanning_state: State<'_, BluetoothLEScanning>,
) -> Result<()> {
    let scanning = *scanning_state.0.lock().unwrap();
    if scanning {
        return Err(bluetoothle::BluetoothLEError::ScanInProgress.into());
    }

    let channel = channel_state.0.lock().await.clone();
    bluetoothle::start_scan(channel, payload)
        .await
        .map_err(|e| Error::from(e))?;

    let mut scanning = scanning_state.0.lock().unwrap();
    *scanning = true;
    Ok(())
}

#[command]
pub(crate) async fn connect<R: Runtime>(
    payload: ConnectRequest,
    _app: AppHandle<R>,
    _window: Window<R>,
    connected_devices_state: State<'_, ConnectedDevices>,
) -> Result<()> {
    let address = payload.address.clone();
    let connected = connected_devices_state.0.lock().unwrap().contains(&address);
    if connected {
        return Err(bluetoothle::BluetoothLEError::AlreadyConnected.into());
    }

    bluetoothle::connect(payload)
        .await
        .map_err(|e| Error::from(e))?;
    connected_devices_state.0.lock().unwrap().push(address);

    Ok(())
}

#[command]
pub(crate) async fn get_services<R: Runtime>(
    payload: GetServicesRequest,
    _app: AppHandle<R>,
    _window: Window<R>,
) -> Result<Vec<Service>> {
    bluetoothle::get_services(payload)
        .await
        .map_err(|e| Error::from(e))
}
