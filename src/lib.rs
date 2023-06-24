use btleplug::platform;
use std::sync::{Arc, Mutex};
use tauri::async_runtime;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod bluetoothle;
mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::TaurineBluetoothLE;
#[cfg(mobile)]
use mobile::TaurineBluetoothLE;

#[cfg(target_os = "android")]
pub(crate) mod android;

use bluetoothle::BluetoothLEMessage;

#[derive(Default)]
struct BluetoothLEScanning(Mutex<bool>);

#[derive(Default)]
struct ConnectedDevices(Mutex<Vec<String>>);

pub struct BluetoothLEChannel(async_runtime::Mutex<async_runtime::Sender<BluetoothLEMessage>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the taurine-bluetooth-le APIs.
pub trait TaurineBluetoothLEExt<R: Runtime> {
    fn taurine_bluetooth_le(&self) -> &TaurineBluetoothLE<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TaurineBluetoothLEExt<R> for T {
    fn taurine_bluetooth_le(&self) -> &TaurineBluetoothLE<R> {
        self.state::<TaurineBluetoothLE<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("taurine-bluetooth-le")
        .invoke_handler(tauri::generate_handler![
            commands::init,
            commands::start_scan,
            commands::connect,
            commands::get_services,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let taurine_bluetooth_le = mobile::init(app, api)?;
            #[cfg(desktop)]
            let taurine_bluetooth_le = desktop::init(app, api)?;
            app.manage(taurine_bluetooth_le);

            let scanning = BluetoothLEScanning::default();
            app.manage(scanning);

            let connected_devices = ConnectedDevices::default();
            app.manage(connected_devices);

            let (tx, mut rx) = async_runtime::channel(1);
            let channel = BluetoothLEChannel(async_runtime::Mutex::new(tx));
            app.manage(channel);

            let app_handle = app.app_handle();
            async_runtime::spawn(async move {
                loop {
                    if let Some(message) = rx.recv().await {
                        match message {
                            BluetoothLEMessage::StopScan => {
                                let scanning_state =
                                    app_handle.try_state::<BluetoothLEScanning>().unwrap();
                                let mut scanning =
                                    scanning_state.0.lock().expect("scanning posisoned");
                                *scanning = false;
                                async_runtime::spawn(async move {
                                    let _ = bluetoothle::stop_scan().await;
                                });
                                let _ = app_handle.emit_all("ble_stop_scan", ());
                            }
                            BluetoothLEMessage::DeviceDiscovered(device) => {
                                let _ = app_handle.emit_all("ble_device_discovered", device);
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .build()
}
