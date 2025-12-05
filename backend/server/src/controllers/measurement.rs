use crate::{
    error::{
        AppError, //
        ResourceError,
    },
    extractors::{
        api_key::AuthDevice, //
        auth::AuthUser,
    },
    models::{
        device::Devices, //
        measurement::Measurements,
    },
    state::AppState,
    views::measurement::{
        MeasurementHistoryQuery, //
        MeasurementResponse,
        SubmitMeasurementRequest,
    },
};

use axum::{
    Json, //
    extract::{Path, Query, State},
    http::StatusCode,
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
/// Requires API Key in header `x-api-key`
#[utoipa::path(
    post,
    path = "/devices/measurements",
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
    State(state): State<AppState>,
    device: AuthDevice,
    Json(payload): Json<SubmitMeasurementRequest>,
) -> Result<StatusCode, AppError> {
    Measurements::add(&state.db, device.id, payload.value, payload.timestamp).await?;

    // TODO: check alerts

    Ok(StatusCode::CREATED)
}

/// Get measurement history
///
/// Accessible by User (owner of the device) via JWT
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
    State(state): State<AppState>,
    user: AuthUser,
    Path(device_id): Path<i32>,
    Query(params): Query<MeasurementHistoryQuery>,
) -> Result<Json<Vec<MeasurementResponse>>, AppError> {
    let _device = Devices::find_by_id_and_user(&state.db, device_id, user.id)
        .await?
        .ok_or_else(|| ResourceError::NotFound {
            msg: "Device not found".into(),
        })?;

    let history = Measurements::find_history(&state.db, device_id, params).await?;

    let response = history
        .into_iter()
        .map(|m| MeasurementResponse {
            time: m.time,
            value: m.value,
            device_id: m.device_id,
        })
        .collect();

    Ok(Json(response))
}
