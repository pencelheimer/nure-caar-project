use crate::{
    error::{
        AppError, //
        AuthError,
    },
    extractors::auth::AuthAdmin,
    models::{
        audit::AuditLogs, //
        system::System,
        user::Users
    },
    state::AppState,
    views::admin::*,
};

use axum::{
    Json, //
    extract::{
        Path, //
        Query,
        State,
    },
    http::StatusCode,
    response::IntoResponse,
};
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn register_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_users))
        .routes(routes!(update_user_role))
        .routes(routes!(get_system_stats))
        .routes(routes!(get_audit_logs))
}

/// List all users
#[utoipa::path(
    get,
    path = "/admin/users",
    responses(
        (status = 200, description = "List of all users", body = Vec<AdminUserDetailsResponse>)
    ),
    tag = "Admin",
    security(("jwt" = []))
)]
pub async fn list_users(
    State(state): State<AppState>,
    _admin: AuthAdmin,
) -> Result<Json<Vec<AdminUserDetailsResponse>>, AppError> {
    let users_with_stats = Users::find_all_with_stats(&state.db).await?;

    let response = users_with_stats
        .into_iter()
        .map(|u| AdminUserDetailsResponse {
            id: u.id,
            email: u.email,
            first_name: u.first_name,
            last_name: u.last_name,
            role: u.role.into(),
            created_at: u.created_at.unwrap_or_default(),
            reservoirs_count: u.reservoirs_count as u64,
            devices_count: u.devices_count as u64,
        })
        .collect();

    Ok(Json(response))
}

/// Update user role
#[utoipa::path(
    put,
    path = "/admin/users/{id}/role",
    params(
        ("id" = i32, Path, description = "User ID"),
        ("role" = UserRole, Query, description = "New role for the User")
    ),
    responses(
        (status = 200, description = "Role updated"),
        (status = 404, description = "User not found"),
        (status = 403, description = "Permission denied")
    ),
    tag = "Admin",
    security(("jwt" = []))
)]
pub async fn update_user_role(
    State(state): State<AppState>,
    admin: AuthAdmin,
    Query(params): Query<UpdateRoleParams>,
    Path(target_user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    if target_user_id == admin.0.id {
        return Err(AuthError::PermissionDenied.into());
    }

    let new_role = params.role.into();
    Users::update_role(&state.db, target_user_id, new_role).await?;

    Ok(StatusCode::OK)
}

/// Get system dashboard statistics
#[utoipa::path(
    get,
    path = "/admin/stats",
    responses(
        (status = 200, description = "System dashboard stats", body = SystemStatsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Permission denied"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Admin",
    security(("jwt" = []))
)]
pub async fn get_system_stats(
    State(state): State<AppState>,
    _admin: AuthAdmin,
) -> Result<Json<SystemStatsResponse>, AppError> {
    let stats = System::get_stats(&state.db).await?;

    Ok(Json(stats))
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
        (status = 200, description = "Audit logs", body = Vec<LogEntryResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Permission denied"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Admin",
    security(("jwt" = []))
)]
pub async fn get_audit_logs(
    State(state): State<AppState>,
    _admin: AuthAdmin,
    Query(params): Query<LogQuery>,
) -> Result<Json<Vec<LogEntryResponse>>, AppError> {
    let logs = AuditLogs::find_filtered(&state.db, params).await?;

    let response = logs
        .into_iter()
        .map(|log| LogEntryResponse {
            id: log.id.to_string(),
            table_name: log.table_name,
            record_id: log.record_id,
            operation: log.operation,
            old_values: log.old_values,
            new_values: log.new_values,
            changed_at: log.changed_at.unwrap(), // there is DEFAULT constraint in the db
        })
        .collect();

    Ok(Json(response))
}
