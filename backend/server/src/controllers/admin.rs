use crate::views::admin::*;
use axum::{
    Json, //
    extract::{
        Path, //
        Query,
    },
    http::StatusCode,
    response::IntoResponse,
};
use utoipa_axum::{router::OpenApiRouter, routes};

/// List all users
#[utoipa::path(
    get,
    path = "/admin/users",
    responses(
        (status = 200, description = "List of all users", body = Vec<AdminUserDetailsResponse>)
    ),
    tag = "Admin",
    security(("jwt" = []))
    // TODO(pencelheimer): pagination?
)]
pub async fn list_users() -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Update user role
#[utoipa::path(
    put,
    path = "/admin/users/{id}/role",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = UpdateUserRoleRequest,
    responses(
        (status = 200, description = "Role updated"),
        (status = 404, description = "User not found")
    ),
    tag = "Admin",
    security(("jwt" = []))
)]
pub async fn update_user_role(
    Path(_id): Path<i32>,
    Json(_payload): Json<UpdateUserRoleRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Get system dashboard statistics
#[utoipa::path(
    get,
    path = "/admin/stats",
    responses(
        (status = 200, description = "System dashboard stats", body = SystemStatsResponse)
    ),
    tag = "Admin",
    security(("jwt" = []))
)]
pub async fn get_system_stats() -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

/// Get audit logs
///
/// Retrieves logs from the `audit_log` table, allowing filtering by table name, operation, etc.
#[utoipa::path(
    get,
    path = "/admin/audit-logs",
    params(
        LogQuery
    ),
    responses(
        (status = 200, description = "Audit logs", body = Vec<LogEntryResponse>)
    ),
    tag = "Admin",
    security(("jwt" = []))
    // TODO(pencelheimer): pagination?
)]
pub async fn get_audit_logs(
    Query(_params): Query<LogQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

pub fn register_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_users))
        .routes(routes!(update_user_role))
        .routes(routes!(get_system_stats))
        .routes(routes!(get_audit_logs))
}
