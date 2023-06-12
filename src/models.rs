use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidInitRequest {
    pub for_location: Option<bool>,
}

impl Default for AndroidInitRequest {
    fn default() -> Self {
        Self {
            for_location: Some(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitRequest {
    pub android: Option<AndroidInitRequest>,
}

impl Default for InitRequest {
    fn default() -> Self {
        Self {
            android: Some(AndroidInitRequest::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanBleRequest {
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanBleResponse;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothLEDevice {
    pub address: String,
    pub name: Option<String>,
    pub uuids: Vec<String>,
}
