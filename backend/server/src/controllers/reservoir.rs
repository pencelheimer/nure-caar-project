use crate::views::reservoir::{
    CreateReservoirRequest, //
    ReservoirResponse,      //
    UpdateReservoirRequest, //
};
use axum::{
    Json,                   //
    extract::Path,          //
    http::StatusCode,       //
    response::IntoResponse, //
};
use utoipa_axum::{
    router::OpenApiRouter, //
    routes,                //
};

pub fn register_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_reservoirs))
        .routes(routes!(create_reservoir))
        .routes(routes!(get_reservoir))
        .routes(routes!(update_reservoir))
        .routes(routes!(delete_reservoir))
}

/// List all reservoirs for current user
#[utoipa::path(
    get,
    path = "/reservoirs",
    responses(
        (status = 200, description = "List of reservoirs", body = Vec<ReservoirResponse>)
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn list_reservoirs() -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Create a new reservoir
#[utoipa::path(
    post,
    path = "/reservoirs",
    request_body = CreateReservoirRequest,
    responses(
        (status = 201, description = "Reservoir created", body = ReservoirResponse)
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn create_reservoir(
    Json(_payload): Json<CreateReservoirRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Get a specific reservoir
#[utoipa::path(
    get,
    path = "/reservoirs/{id}",
    params(
        ("id" = i32, Path, description = "Reservoir ID")
    ),
    responses(
        (status = 200, description = "Reservoir details", body = ReservoirResponse),
        (status = 404, description = "Reservoir not found")
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn get_reservoir(Path(_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Update a reservoir
#[utoipa::path(
    put,
    path = "/reservoirs/{id}",
    params(
        ("id" = i32, Path, description = "Reservoir ID")
    ),
    request_body = UpdateReservoirRequest,
    responses(
        (status = 200, description = "Reservoir updated", body = ReservoirResponse),
        (status = 404, description = "Reservoir not found")
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn update_reservoir(
    Path(_id): Path<i32>,
    Json(_payload): Json<UpdateReservoirRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Delete a reservoir
#[utoipa::path(
    delete,
    path = "/reservoirs/{id}",
    params(
        ("id" = i32, Path, description = "Reservoir ID")
    ),
    responses(
        (status = 204, description = "Reservoir deleted"),
        (status = 404, description = "Reservoir not found")
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn delete_reservoir(Path(_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}
