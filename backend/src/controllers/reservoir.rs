use crate::{
    error::{AppError, ResourceError},
    extractors::auth::AuthUser,
    models::reservoir::Reservoirs,
    state::AppState,
    views::reservoir::{
        CreateReservoirRequest, //
        ReservoirResponse,
        UpdateReservoirRequest,
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
        (status = 200, description = "List of reservoirs", body = Vec<ReservoirResponse>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn list_reservoirs(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Vec<ReservoirResponse>>, AppError> {
    let reservoirs = Reservoirs::find_all_by_user(&state.db, user.id).await?;

    let response = reservoirs
        .into_iter()
        .map(|r| ReservoirResponse {
            id: r.id,
            name: r.name,
            description: r.description,
            capacity: r.capacity,
            location: r.location,
        })
        .collect();

    Ok(Json(response))
}

/// Create a new reservoir
#[utoipa::path(
    post,
    path = "/reservoirs",
    request_body = CreateReservoirRequest,
    responses(
        (status = 201, description = "Reservoir created", body = ReservoirResponse),
        (status = 400, description = "Invalid data"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn create_reservoir(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateReservoirRequest>,
) -> Result<(StatusCode, Json<ReservoirResponse>), AppError> {
    let created = Reservoirs::create(&state.db, user.id, payload).await?;

    let response = ReservoirResponse {
        id: created.id,
        name: created.name,
        description: created.description,
        capacity: created.capacity,
        location: created.location,
    };

    Ok((StatusCode::CREATED, Json(response)))
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
        (status = 404, description = "Reservoir not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn get_reservoir(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ReservoirResponse>, AppError> {
    let reservoir = Reservoirs::find_by_id_and_user(&state.db, id, user.id)
        .await?
        .ok_or_else(|| ResourceError::NotFound {
            msg: "Reservoir not found".into(),
        })?;

    Ok(Json(ReservoirResponse {
        id: reservoir.id,
        name: reservoir.name,
        description: reservoir.description,
        capacity: reservoir.capacity,
        location: reservoir.location,
    }))
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
        (status = 404, description = "Reservoir not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn update_reservoir(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateReservoirRequest>,
) -> Result<Json<ReservoirResponse>, AppError> {
    let updated = Reservoirs::update(&state.db, id, user.id, payload).await?;

    Ok(Json(ReservoirResponse {
        id: updated.id,
        name: updated.name,
        description: updated.description,
        capacity: updated.capacity,
        location: updated.location,
    }))
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
        (status = 404, description = "Reservoir not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Reservoirs",
    security(("jwt" = []))
)]
pub async fn delete_reservoir(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    Reservoirs::delete(&state.db, id, user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
