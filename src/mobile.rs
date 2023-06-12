use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "android")]
use crate::android::*;

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "io.taurine.bluetooth_le.plugin";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_taurine - bluetooth - le);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<TaurineBluetoothLE<R>> {
    #[cfg(target_os = "android")]
    {
        create_runtime()?;
        let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "TaurineBluetoothLEPlugin")?;
        Ok(TaurineBluetoothLE(handle))
    }
    #[cfg(target_os = "ios")]
    {
        let handle = api.register_ios_plugin(init_plugin_taurine - bluetooth - le)?;
        Ok(TaurineBluetoothLE(handle))
    }
}

/// Access to the taurine-bluetooth-le APIs.
pub struct TaurineBluetoothLE<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> TaurineBluetoothLE<R> {
    #[cfg(target_os = "android")]
    pub fn init_ble(&self, options: AndroidInitRequest) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("initBle", options)
            .map_err(Into::into)
    }

    #[cfg(target_os = "ios")]
    pub fn init_ble(&self) -> crate::Result<()> {
        self.0.run_mobile_plugin("initBle", ()).map_err(Into::into)
    }
}
