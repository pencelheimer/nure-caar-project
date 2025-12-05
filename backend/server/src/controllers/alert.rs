use crate::{
    error::AppError, //
    extractors::auth::AuthUser,
    models::alert::Alerts,
    state::AppState,
    views::alert::*,
};

use axum::{
    Json,
    extract::{
        Path, //
        Query,
        State,
    },
    http::StatusCode,
};
use utoipa_axum::{
    router::OpenApiRouter, //
    routes,
};

pub fn register_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_rules))
        .routes(routes!(create_rule))
        .routes(routes!(update_rule))
        .routes(routes!(delete_rule))
        .routes(routes!(get_alert_history))
}

/// Get rules for a reservoir
#[utoipa::path(
    get,
    path = "/reservoirs/{id}/rules",
    params(("id" = i32, Path, description = "Reservoir ID")),
    responses(
        (status = 200, description = "List of rules", body = Vec<AlertRuleResponse>),
        (status = 404, description = "Reservoir not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Alerts",
    security(("jwt" = []))
)]
pub async fn list_rules(
    State(state): State<AppState>,
    user: AuthUser,
    Path(reservoir_id): Path<i32>,
) -> Result<Json<Vec<AlertRuleResponse>>, AppError> {
    let rules = Alerts::find_rules_by_reservoir(&state.db, reservoir_id, user.id).await?;

    let response = rules.into_iter().map(Into::into).collect();

    Ok(Json(response))
}

/// Create a new rule
#[utoipa::path(
    post,
    path = "/reservoirs/{id}/rules",
    params(("id" = i32, Path, description = "Reservoir ID")),
    request_body = CreateAlertRuleRequest,
    responses(
        (status = 201, description = "Rule created", body = AlertRuleResponse),
        (status = 404, description = "Reservoir not found")
    ),
    tag = "Alerts",
    security(("jwt" = []))
)]
pub async fn create_rule(
    State(state): State<AppState>,
    user: AuthUser,
    Path(reservoir_id): Path<i32>,
    Json(payload): Json<CreateAlertRuleRequest>,
) -> Result<(StatusCode, Json<AlertRuleResponse>), AppError> {
    let created = Alerts::create_rule(
        &state.db,
        reservoir_id,
        user.id,
        payload.condition_type.into(),
        payload.threshold,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(created.into())))
}

/// Update a rule
#[utoipa::path(
    put,
    path = "/rules/{id}",
    params(("id" = i32, Path, description = "Rule ID")),
    request_body = UpdateAlertRuleRequest,
    responses(
        (status = 200, description = "Rule updated", body = AlertRuleResponse),
        (status = 404, description = "Rule not found")
    ),
    tag = "Alerts",
    security(("jwt" = []))
)]
pub async fn update_rule(
    State(state): State<AppState>,
    user: AuthUser,
    Path(rule_id): Path<i32>,
    Json(payload): Json<UpdateAlertRuleRequest>,
) -> Result<Json<AlertRuleResponse>, AppError> {
    let db_condition = payload.condition_type.map(|c| c.into());

    let updated = Alerts::update_rule(
        &state.db,
        rule_id,
        user.id,
        db_condition,
        payload.threshold,
        payload.is_active,
    )
    .await?;

    Ok(Json(updated.into()))
}

/// Delete a rule
#[utoipa::path(
    delete,
    path = "/rules/{id}",
    params(("id" = i32, Path, description = "Rule ID")),
    responses(
        (status = 204, description = "Rule deleted"),
        (status = 404, description = "Rule not found")
    ),
    tag = "Alerts",
    security(("jwt" = []))
)]
pub async fn delete_rule(
    State(state): State<AppState>,
    user: AuthUser,
    Path(rule_id): Path<i32>,
) -> Result<StatusCode, AppError> {
    Alerts::delete_rule(&state.db, rule_id, user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Get alert logs
#[utoipa::path(
    get,
    path = "/alerts",
    params(AlertHistoryQuery),
    responses(
        (status = 200, description = "Alert logs", body = Vec<AlertLogResponse>)
    ),
    tag = "Alerts",
    security(("jwt" = []))
)]
pub async fn get_alert_history(
    State(state): State<AppState>,
    user: AuthUser,
    Query(params): Query<AlertHistoryQuery>,
) -> Result<Json<Vec<AlertLogResponse>>, AppError> {
    let logs = Alerts::find_history_by_user(&state.db, user.id, params).await?;

    let response = logs.into_iter().map(Into::into).collect();

    Ok(Json(response))
}
