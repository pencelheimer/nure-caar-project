use crate::{
    state::AppState, //
    views::auth::{
        AuthResponse, //
        LoginRequest,
        RegisterRequest,
    },
};
use axum::{
    Json, //
    http::StatusCode,
    response::IntoResponse,
};
use utoipa_axum::{
    router::OpenApiRouter, //
    routes,
};

pub fn register_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(register))
        .routes(routes!(login))
}

/// Register a new user
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Bad request (e.g. email exists)")
    ),
    tag = "Auth"
)]
#[axum::debug_handler]
pub async fn register(
    Json(_payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Login user
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Auth"
)]
#[axum::debug_handler]
pub async fn login(Json(_payload): Json<LoginRequest>) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}
