use crate::state::AppState;
use axum::{
    Json, //
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use utoipa_axum::{
    router::OpenApiRouter, //
    routes,
};

pub fn register_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(health_check))
}

/// Health Check
///
/// Simple endpoint to check if the server is up and running.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = inline(serde_json::Value))
    ),
    tag = "System"
)]
pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "status": "ok", "timestamp": chrono::Utc::now() })),
    )
}
