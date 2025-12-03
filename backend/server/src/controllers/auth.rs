use crate::{
    error::{AppError, AuthError},
    models::user::Users,
    state::AppState,
    utils::jwt,
    views::auth::{
        AuthResponse, //
        LoginRequest,
        RegisterRequest,
    },
};
use axum::{
    Json, //
    extract::State,
    http::StatusCode,
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
        (status = 400, description = "Bad request (e.g. email exists or invalid format)"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Auth"
)]
#[axum::debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    if Users::exists_by_email(&state.db, &payload.email).await? {
        return Err(AuthError::UserAlreadyExists.into());
    }

    let hashed_password = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)?;

    let user = Users::create(
        &state.db,
        payload.email,
        hashed_password,
        payload.first_name,
        payload.last_name,
        None,
    )
    .await?;

    let token = jwt::sign(user.id, &user.email, &state.config.jwt_secret)?;

    let response = AuthResponse {
        token,
        user_id: user.id,
        email: user.email,
    };

    Ok((StatusCode::CREATED, Json(response)))
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
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = Users::find_by_email(&state.db, &payload.email)
        .await?
        .ok_or(AuthError::WrongCredentials)?;

    if !bcrypt::verify(&payload.password, &user.hashed_password)? {
        return Err(AuthError::WrongCredentials.into());
    }

    let token = jwt::sign(user.id, &user.email, &state.config.jwt_secret)?;

    let response = AuthResponse {
        token,
        user_id: user.id,
        email: user.email,
    };

    Ok(Json(response))
}
