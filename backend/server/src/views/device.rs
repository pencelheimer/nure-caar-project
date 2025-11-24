use chrono::{
    DateTime,    //
    FixedOffset, //
};
use serde::{
    Deserialize, //
    Serialize,   //
};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateDeviceRequest {
    #[schema(example = "ESP32 Sensor 01")]
    pub name: String,
    // reservoir_id is optional upon creation, can be assigned later
    pub reservoir_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateDeviceRequest {
    pub name: Option<String>,
    pub reservoir_id: Option<i32>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeviceResponse {
    pub id: i32,
    pub name: String,
    pub reservoir_id: Option<i32>,
    pub status: Option<String>,
    pub last_seen: Option<DateTime<FixedOffset>>,
    // We usually don't return the full API key in a list response for security,
    // but might return a masked version or just for the owner. TODO
    pub api_key_masked: String,
}
