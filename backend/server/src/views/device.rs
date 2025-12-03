use chrono::{
    DateTime, //
    FixedOffset,
};
use serde::{
    Deserialize, //
    Serialize,
};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeviceStatus {
    Online,
    Offline,
    Maintenance,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateDeviceRequest {
    #[schema(example = "ESP32 Sensor 01")]
    pub name: String,
    pub reservoir_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateDeviceRequest {
    pub name: Option<String>,
    pub reservoir_id: Option<i32>,
    pub status: Option<DeviceStatus>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeviceResponse {
    pub id: i32,
    pub name: String,
    pub reservoir_id: Option<i32>,
    pub status: DeviceStatus,
    pub last_seen: Option<DateTime<FixedOffset>>,
    pub api_key_masked: String,
}
