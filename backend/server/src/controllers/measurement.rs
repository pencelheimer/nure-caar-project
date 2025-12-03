use crate::{
    state::AppState, //
    views::measurement::{
        MeasurementHistoryQuery, //
        MeasurementResponse,
        SubmitMeasurementRequest,
    },
};
use axum::{
    Json, //
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use utoipa_axum::{
    router::OpenApiRouter, //
    routes,
};

pub fn register_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(submit_measurement))
        .routes(routes!(get_measurements))
}

/// Submit measurement data (from Device)
///
/// Requires API Key in header, not JWT
#[utoipa::path(
    post,
    path = "/devices/{id}/measurements",
    params(
        ("id" = i32, Path, description = "Device ID")
    ),
    request_body = SubmitMeasurementRequest,
    responses(
        (status = 201, description = "Measurement accepted"),
        (status = 404, description = "Device not found"),
        (status = 403, description = "Invalid API Key")
    ),
    tag = "Measurements",
    security(("api_key" = []))
)]
pub async fn submit_measurement(
    Path(_id): Path<i32>,
    Json(_payload): Json<SubmitMeasurementRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Get measurement history
#[utoipa::path(
    get,
    path = "/devices/{id}/measurements",
    params(
        ("id" = i32, Path, description = "Device ID"),
        MeasurementHistoryQuery
    ),
    responses(
        (status = 200, description = "History data", body = Vec<MeasurementResponse>)
    ),
    tag = "Measurements",
    security(("jwt" = []))
)]
pub async fn get_measurements(
    Path(_id): Path<i32>,
    Query(_params): Query<MeasurementHistoryQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}
