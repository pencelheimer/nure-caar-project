use chrono::{
    DateTime,    //
    FixedOffset, //
};
use serde::{
    Deserialize, //
    Serialize,   //
};
use utoipa::{
    IntoParams, //
    ToSchema,   //
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SubmitMeasurementRequest {
    #[schema(example = 450.5)]
    pub value: f64,
    // if not provided, server uses current time
    pub timestamp: Option<DateTime<FixedOffset>>,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct MeasurementHistoryQuery {
    pub from: Option<DateTime<FixedOffset>>,
    pub to: Option<DateTime<FixedOffset>>,
    #[param(default = 100)]
    pub limit: Option<u64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MeasurementResponse {
    pub time: DateTime<FixedOffset>,
    pub value: f64,
    pub device_id: i32,
}
