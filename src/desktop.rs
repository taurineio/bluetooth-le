use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<TaurineBluetoothLE<R>> {
    Ok(TaurineBluetoothLE(app.clone()))
}

/// Access to the taurine-bluetooth-le APIs.
pub struct TaurineBluetoothLE<R: Runtime>(AppHandle<R>);

impl<R: Runtime> TaurineBluetoothLE<R> {
    pub fn init_ble(&self) -> crate::Result<()> {
        Ok(())
    }
}
