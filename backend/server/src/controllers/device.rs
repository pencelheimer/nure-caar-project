use crate::views::device::{
    CreateDeviceRequest, //
    DeviceResponse,
    UpdateDeviceRequest,
};
use axum::{
    Json, //
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
};
use utoipa_axum::{
    router::OpenApiRouter, //
    routes,
};

pub fn register_routes() -> OpenApiRouter {
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
        (status = 200, description = "List of devices", body = Vec<DeviceResponse>)
    ),
    tag = "Devices",
    security(("jwt" = []))
)]
pub async fn list_devices() -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Create a new device
#[utoipa::path(
    post,
    path = "/devices",
    request_body = CreateDeviceRequest,
    responses(
        (status = 201, description = "Device created", body = DeviceResponse)
    ),
    tag = "Devices",
    security(("jwt" = []))
)]
pub async fn create_device(
    Json(_payload): Json<CreateDeviceRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Update device
///
/// E.g. update a link to reservoir
#[utoipa::path(
    put,
    path = "/devices/{id}",
    params(
        ("id" = i32, Path, description = "Device ID")
    ),
    request_body = UpdateDeviceRequest,
    responses(
        (status = 200, description = "Device updated", body = DeviceResponse),
        (status = 404, description = "Device not found")
    ),
    tag = "Devices",
    security(("jwt" = []))
)]
pub async fn update_device(
    Path(_id): Path<i32>,
    Json(_payload): Json<UpdateDeviceRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
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
        (status = 404, description = "Device not found")
    ),
    tag = "Devices",
    security(("jwt" = []))
)]
pub async fn delete_device(Path(_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}
