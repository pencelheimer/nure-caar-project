use crate::models::entities::sea_orm_active_enums::DeviceStatus as DbDeviceStatus;

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

impl From<DbDeviceStatus> for DeviceStatus {
    fn from(val: DbDeviceStatus) -> Self {
        match val {
            DbDeviceStatus::Online => Self::Online,
            DbDeviceStatus::Offline => Self::Offline,
            DbDeviceStatus::Maintenance => Self::Maintenance,
        }
    }
}

impl From<DeviceStatus> for DbDeviceStatus {
    fn from(val: DeviceStatus) -> Self {
        match val {
            DeviceStatus::Online => Self::Online,
            DeviceStatus::Offline => Self::Offline,
            DeviceStatus::Maintenance => Self::Maintenance,
        }
    }
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
    pub reservoir_id: Option<Option<i32>>,
    pub status: Option<DeviceStatus>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeviceResponse {
    pub id: i32,
    pub name: String,
    pub reservoir_id: Option<i32>,
    pub status: DeviceStatus,
    pub last_seen: Option<DateTime<FixedOffset>>,
    pub api_key: String,
}
