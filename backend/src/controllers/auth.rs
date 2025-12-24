use crate::{
    error::{AppError, AuthError}, extractors::auth::AuthUser, models::user::Users, state::AppState, utils::jwt, views::auth::{
        AuthResponse, ChangePasswordRequest, LoginRequest, RegisterRequest, UpdateProfileRequest,
        UserProfileResponse,
    }
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
        .routes(routes!(get_me))
        .routes(routes!(update_me))
        .routes(routes!(change_password))
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

/// Get current user profile
#[utoipa::path(
    get,
    path = "/auth/me",
    responses(
        (status = 200, description = "Current user profile", body = UserProfileResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Auth",
    security(("jwt" = []))
)]
pub async fn get_me(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<UserProfileResponse>, AppError> {
    let user_model = Users::get_by_id(&state.db, user.id).await?;

    Ok(Json(UserProfileResponse {
        id: user_model.id,
        email: user_model.email,
        first_name: user_model.first_name,
        last_name: user_model.last_name,
        role: user_model.role.into(),
        created_at: user_model.created_at.unwrap_or_default(),
    }))
}

/// Update current user profile
#[utoipa::path(
    put,
    path = "/auth/me",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated", body = UserProfileResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Auth",
    security(("jwt" = []))
)]
pub async fn update_me(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfileResponse>, AppError> {
    let updated_user = Users::update_profile(&state.db, user.id, payload).await?;

    Ok(Json(UserProfileResponse {
        id: updated_user.id,
        email: updated_user.email,
        first_name: updated_user.first_name,
        last_name: updated_user.last_name,
        role: updated_user.role.into(),
        created_at: updated_user.created_at.unwrap_or_default(),
    }))
}

/// Change password
#[utoipa::path(
    post,
    path = "/auth/change-password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Invalid current password"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Auth",
    security(("jwt" = []))
)]
pub async fn change_password(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, AppError> {
    let user_model = Users::get_by_id(&state.db, user.id).await?;

    let valid = bcrypt::verify(&payload.current_password, &user_model.hashed_password)?;
    if !valid {
        return Err(AuthError::WrongCredentials.into());
    }

    let new_hash = bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST)?;

    Users::update_password(&state.db, user.id, new_hash).await?;

    Ok(StatusCode::OK)
}
