use crate::{
    error::AppError,
    extractors::auth::AuthUser,
    models::push_token::PushTokens,
    state::AppState,
    views::push_token::{DeletePushTokenRequest, PushTokenResponse, RegisterPushTokenRequest},
};

use axum::{Json, extract::State, http::StatusCode};
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn register_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(register_token))
        .routes(routes!(delete_token))
}

/// Register or refresh FCM push token for the authenticated user
#[utoipa::path(
    post,
    path = "/users/push-token",
    request_body = RegisterPushTokenRequest,
    responses(
        (status = 200, description = "Token registered", body = PushTokenResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Push",
    security(("jwt" = []))
)]
pub async fn register_token(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<RegisterPushTokenRequest>,
) -> Result<Json<PushTokenResponse>, AppError> {
    PushTokens::upsert(&state.db, user.id, payload.token, payload.platform).await?;
    Ok(Json(PushTokenResponse { registered: true }))
}

/// Remove FCM push token for the authenticated user
#[utoipa::path(
    delete,
    path = "/users/push-token",
    request_body = DeletePushTokenRequest,
    responses(
        (status = 204, description = "Token removed"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Push",
    security(("jwt" = []))
)]
pub async fn delete_token(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<DeletePushTokenRequest>,
) -> Result<StatusCode, AppError> {
    PushTokens::delete(&state.db, user.id, &payload.token).await?;
    Ok(StatusCode::NO_CONTENT)
}
