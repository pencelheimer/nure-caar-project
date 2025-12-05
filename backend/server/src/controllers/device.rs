use crate::{
    error::AppError,
    extractors::auth::AuthUser,
    models::device::Devices,
    state::AppState,
    utils::fns::masked_api_key,
    views::device::{
        CreateDeviceRequest, //
        DeviceResponse,
        DeviceStatus,
        UpdateDeviceRequest,
    },
};

use axum::{
    Json, //
    extract::{Path, State},
    http::StatusCode,
};
use utoipa_axum::{
    router::OpenApiRouter, //
    routes,
};

pub fn register_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_devices))
        .routes(routes!(create_device))
        .routes(routes!(update_device))
        .routes(routes!(delete_device))
}

/// List all devices
#[utoipa::path(
    get,
    path = "/devices",
    responses(
        (status = 200, description = "List of devices", body = Vec<DeviceResponse>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Devices",
    security(("jwt" = []))
)]
pub async fn list_devices(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Vec<DeviceResponse>>, AppError> {
    let devices = Devices::find_all_by_user(&state.db, user.id).await?;

    let response = devices
        .into_iter()
        .map(|d| {
            let key_masked = masked_api_key(d.api_key.as_str());

            DeviceResponse {
                id: d.id,
                name: d.name,
                reservoir_id: d.reservoir_id,
                status: d.status.into(),
                last_seen: d.last_seen,
                api_key: key_masked,
            }
        })
        .collect();

    Ok(Json(response))
}

/// Create a new device
#[utoipa::path(
    post,
    path = "/devices",
    request_body = CreateDeviceRequest,
    responses(
        (status = 201, description = "Device created", body = DeviceResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Devices",
    security(("jwt" = []))
)]
pub async fn create_device(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateDeviceRequest>,
) -> Result<(StatusCode, Json<DeviceResponse>), AppError> {
    let created = Devices::create(&state.db, user.id, payload).await?;

    let response = DeviceResponse {
        id: created.id,
        name: created.name,
        reservoir_id: created.reservoir_id,
        status: DeviceStatus::Offline,
        last_seen: None,
        api_key: created.api_key,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Update device
#[utoipa::path(
    put,
    path = "/devices/{id}",
    params(
        ("id" = i32, Path, description = "Device ID")
    ),
    request_body = UpdateDeviceRequest,
    responses(
        (status = 200, description = "Device updated", body = DeviceResponse),
        (status = 404, description = "Device not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Devices",
    security(("jwt" = []))
)]
pub async fn update_device(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateDeviceRequest>,
) -> Result<Json<DeviceResponse>, AppError> {
    let updated = Devices::update(&state.db, id, user.id, payload).await?;

    let response = DeviceResponse {
        id: updated.id,
        name: updated.name,
        reservoir_id: updated.reservoir_id,
        status: updated.status.into(),
        last_seen: updated.last_seen,
        api_key: masked_api_key(updated.api_key.as_str()),
    };

    Ok(Json(response))
}

/// Delete a device
#[utoipa::path(
    delete,
    path = "/devices/{id}",
    params(
        ("id" = i32, Path, description = "Device ID")
    ),
    responses(
        (status = 204, description = "Device deleted"),
        (status = 404, description = "Device not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Devices",
    security(("jwt" = []))
)]
pub async fn delete_device(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    Devices::delete(&state.db, id, user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
