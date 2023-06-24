use btleplug::api::{ScanFilter, self};
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

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanBleRequest {
    pub filter: Option<ScanFilter>,
    pub timeout: Option<u64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectRequest {
    pub address: String,
    pub timeout: Option<u64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServicesRequest {
    pub address: String,
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

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Descriptor {
    pub uuid: String,
    pub characteristic_uuid: String,
    pub service_uuid: String,
}

impl From<api::Descriptor> for Descriptor {
    fn from(value: api::Descriptor) -> Self {
        Self {
            uuid: value.uuid.to_string(),
            characteristic_uuid: value.characteristic_uuid.to_string(),
            service_uuid: value.service_uuid.to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    pub uuid: String,
    pub service_uuid: String,
    pub properties: u8,
    pub descriptors: Vec<Descriptor>,
}

impl From<api::Characteristic> for Characteristic {
    fn from(value: api::Characteristic) -> Self {
        Self {
            uuid: value.uuid.to_string(),
            service_uuid: value.service_uuid.to_string(),
            properties: value.properties.bits(),
            descriptors: value.descriptors.into_iter().map(|d| d.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub uuid: String,
    pub primary: bool,
    pub characteristics: Vec<Characteristic>,
}

impl From<api::Service> for Service {
    fn from(value: api::Service) -> Self {
        Self {
            uuid: value.uuid.to_string(),
            primary: value.primary,
            characteristics: value.characteristics.into_iter().map(|c| c.into()).collect(),
        }
    }
}
