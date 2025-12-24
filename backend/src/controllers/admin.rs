use crate::{
    error::{
        AppError, //
        AuthError,
    },
    extractors::auth::AuthAdmin,
    models::{
        audit::AuditLogs, //
        system::System,
        user::Users,
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
        .routes(routes!(get_user_details))
        .routes(routes!(ban_user))
        .routes(routes!(delete_user))
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

    let response = users_with_stats.into_iter().map(Into::into).collect();

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

    let response = logs.into_iter().map(Into::into).collect();

    Ok(Json(response))
}

/// Get user details by ID
#[utoipa::path(
    get,
    path = "/admin/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details", body = AdminUserDetailsResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Admin",
    security(("jwt" = []))
)]
pub async fn get_user_details(
    State(state): State<AppState>,
    _admin: AuthAdmin,
    Path(id): Path<i32>,
) -> Result<Json<AdminUserDetailsResponse>, AppError> {
    let user = Users::find_by_id_with_stats(&state.db, id)
        .await?
        .ok_or(AuthError::UserNotFound)?;

    Ok(Json(user.into()))
}

/// Ban or Unban a user
#[utoipa::path(
    post,
    path = "/admin/users/{id}/ban",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = BanUserRequest,
    responses(
        (status = 200, description = "User ban status updated"),
        (status = 404, description = "User not found"),
        (status = 403, description = "Cannot ban yourself")
    ),
    tag = "Admin",
    security(("jwt" = []))
)]
pub async fn ban_user(
    State(state): State<AppState>,
    admin: AuthAdmin,
    Path(target_id): Path<i32>,
    Json(payload): Json<BanUserRequest>,
) -> Result<StatusCode, AppError> {
    if target_id == admin.0.id {
        return Err(AuthError::PermissionDenied)?;
    }

    Users::set_ban_status(&state.db, target_id, payload.is_banned, payload.ban_reason).await?;

    Ok(StatusCode::OK)
}

/// Delete a user
#[utoipa::path(
    delete,
    path = "/admin/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found"),
        (status = 403, description = "Cannot delete yourself")
    ),
    tag = "Admin",
    security(("jwt" = []))
)]
pub async fn delete_user(
    State(state): State<AppState>,
    admin: AuthAdmin,
    Path(target_id): Path<i32>,
) -> Result<StatusCode, AppError> {
    if target_id == admin.0.id {
        return Err(AuthError::PermissionDenied)?;
    }

    Users::delete(&state.db, target_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
