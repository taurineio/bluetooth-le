use tauri::{command, AppHandle, Runtime, State, Window};

use crate::{
    bluetoothle, models::*, BluetoothLEChannel, BluetoothLEScanning, Error, Result,
    TaurineBluetoothLEExt,
};

#[command]
pub(crate) async fn start_scan<R: Runtime>(
    payload: ScanBleRequest,
    _app: AppHandle<R>,
    _window: Window<R>,
    channel_state: State<'_, BluetoothLEChannel>,
    scanning_state: State<'_, BluetoothLEScanning>,
) -> Result<()> {
    let scanning = *scanning_state.0.lock().unwrap();
    if scanning {
        return Err(bluetoothle::BluetoothLEError::ScanInProgress.into());
    }

    let channel = channel_state.0.lock().await;
    let channel = channel.clone();
    let ScanBleRequest { timeout } = payload;
    bluetoothle::start_scan(channel, timeout)
        .await
        .map_err(|e| Error::from(e))?;

    let mut scanning = scanning_state.0.lock().unwrap();
    *scanning = true;
    Ok(())
}

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
