use crate::{
    error::{
        AppError, //
        AuthError,
        ResourceError,
    },
    models::{
        entities::{
            alert, //
            alert_rule,
            prelude::*,
            reservoir,
            sea_orm_active_enums::{
                AlertConditionType, //
                AlertStatus,
            },
        },
        push_token::PushTokens,
    },
    services::notification::NotificationService,
    views::alert::AlertHistoryQuery,
};

use chrono::Utc;
use sea_orm::*;

pub struct Alerts;

impl Alerts {
    pub async fn find_rules_by_reservoir(
        db: &DatabaseConnection,
        reservoir_id: i32,
        user_id: i32,
    ) -> Result<Vec<alert_rule::Model>, AppError> {
        let reservoir =
            Reservoir::find_by_id(reservoir_id)
                .one(db)
                .await?
                .ok_or(ResourceError::NotFound {
                    msg: "Reservoir not found".into(),
                })?;

        if reservoir.user_id != user_id {
            return Err(AuthError::PermissionDenied)?;
        }

        let rules = AlertRule::find()
            .filter(alert_rule::Column::ReservoirId.eq(reservoir_id))
            .all(db)
            .await?;

        Ok(rules)
    }

    pub async fn create_rule(
        db: &DatabaseConnection,
        reservoir_id: i32,
        user_id: i32,
        condition: AlertConditionType,
        threshold: f64,
    ) -> Result<alert_rule::Model, AppError> {
        let reservoir =
            Reservoir::find_by_id(reservoir_id)
                .one(db)
                .await?
                .ok_or(ResourceError::NotFound {
                    msg: "Reservoir not found".into(),
                })?;

        if reservoir.user_id != user_id {
            return Err(AuthError::PermissionDenied)?;
        }

        let active_model = alert_rule::ActiveModel {
            reservoir_id: Set(reservoir_id),
            condition_type: Set(condition),
            threshold: Set(threshold),
            is_active: Set(Some(true)),
            ..Default::default()
        };

        let res = active_model.insert(db).await?;
        Ok(res)
    }

    pub async fn update_rule(
        db: &DatabaseConnection,
        rule_id: i32,
        user_id: i32,
        condition: Option<AlertConditionType>,
        threshold: Option<f64>,
        is_active: Option<bool>,
    ) -> Result<alert_rule::Model, AppError> {
        let rule_with_res = AlertRule::find_by_id(rule_id)
            .find_also_related(Reservoir)
            .one(db)
            .await?;

        let (rule, reservoir) = match rule_with_res {
            Some((r, Some(res))) => (r, res),
            _ => {
                return Err(ResourceError::NotFound {
                    msg: "Rule not found".into(),
                })?;
            }
        };

        if reservoir.user_id != user_id {
            return Err(AuthError::PermissionDenied.into());
        }

        let mut active: alert_rule::ActiveModel = rule.into();

        if let Some(c) = condition {
            active.condition_type = Set(c);
        }
        if let Some(t) = threshold {
            active.threshold = Set(t);
        }
        if let Some(a) = is_active {
            active.is_active = Set(Some(a));
        }

        let updated = active.update(db).await?;
        Ok(updated)
    }

    pub async fn delete_rule(
        db: &DatabaseConnection,
        rule_id: i32,
        user_id: i32,
    ) -> Result<(), AppError> {
        let rule_with_res = AlertRule::find_by_id(rule_id)
            .find_also_related(Reservoir)
            .one(db)
            .await?;

        let (rule, reservoir) = match rule_with_res {
            Some((r, Some(res))) => (r, res),
            _ => {
                return Err(ResourceError::NotFound {
                    msg: "Rule not found".into(),
                })?;
            }
        };

        if reservoir.user_id != user_id {
            return Err(AuthError::PermissionDenied)?;
        }

        rule.delete(db).await?;
        Ok(())
    }

    pub async fn find_history_by_user(
        db: &DatabaseConnection,
        user_id: i32,
        params: AlertHistoryQuery,
    ) -> Result<Vec<(alert::Model, alert_rule::Model)>, AppError> {
        let alerts = Alert::find()
            .join(JoinType::InnerJoin, alert::Relation::AlertRule.def())
            .join(JoinType::InnerJoin, alert_rule::Relation::Reservoir.def())
            .filter(reservoir::Column::UserId.eq(user_id))
            .order_by_desc(alert::Column::TriggeredAt)
            .limit(params.limit.unwrap_or(50))
            .offset(params.offset.unwrap_or(0))
            .all(db)
            .await?;

        if alerts.is_empty() {
            return Ok(vec![]);
        }

        let rule_ids: Vec<i32> = alerts.iter().map(|a| a.rule_id).collect();
        let rules = AlertRule::find()
            .filter(alert_rule::Column::Id.is_in(rule_ids))
            .all(db)
            .await?;

        let pairs = alerts
            .into_iter()
            .filter_map(|a| {
                rules.iter().find(|r| r.id == a.rule_id).map(|r| (a, r.clone()))
            })
            .collect();

        Ok(pairs)
    }

    pub async fn check_and_notify(
        state: &crate::state::AppState,
        reservoir_id: i32,
        value: f64,
    ) -> Result<(), AppError> {
        let db = &state.db;

        let result = Reservoir::find_by_id(reservoir_id)
            .find_also_related(User)
            .one(db)
            .await?;

        let (reservoir, user) = match result {
            Some((r, Some(u))) => (r, u),
            _ => return Ok(()),
        };

        let user_email = user.email;

        let rules = AlertRule::find()
            .filter(alert_rule::Column::ReservoirId.eq(reservoir_id))
            .filter(alert_rule::Column::IsActive.eq(true))
            .all(db)
            .await?;

        for rule in rules {
            let triggered = match rule.condition_type {
                AlertConditionType::GreaterThan => value > rule.threshold,
                AlertConditionType::LessThan => value < rule.threshold,
                AlertConditionType::Equals => (value - rule.threshold).abs() < f64::EPSILON,
            };

            if triggered {
                let condition_str = match rule.condition_type {
                    AlertConditionType::LessThan => "less_than",
                    AlertConditionType::GreaterThan => "greater_than",
                    AlertConditionType::Equals => "equals",
                };

                let email_subject = "SmartTank Alert";
                let email_body = format!(
                    "Alert for {}: value {:.2} L is {} threshold {:.2} L",
                    reservoir.name, value, condition_str, rule.threshold
                );

                let send_result =
                    NotificationService::send_email(&user_email, email_subject, &email_body).await;

                let status = match send_result {
                    Ok(_) => AlertStatus::Sent,
                    Err(e) => {
                        tracing::error!("Failed to send alert email: {:?}", e);
                        AlertStatus::Failed
                    }
                };

                // Push notification — data-only so Android formats in device locale
                if let Some(ref fcm) = state.fcm {
                    if let Ok(tokens) = PushTokens::find_tokens_by_user(db, user.id).await {
                        fcm.send_alert_push(
                            &tokens,
                            reservoir_id,
                            &reservoir.name,
                            condition_str,
                            rule.threshold,
                            value,
                        )
                        .await;
                    }
                }

                let alert_log = alert::ActiveModel {
                    rule_id: Set(rule.id),
                    sent_to: Set(user_email.clone()),
                    status: Set(status),
                    triggered_at: Set(Utc::now().into()),
                    value: Set(value),
                    ..Default::default()
                };

                alert_log.insert(db).await?;
            }
        }

        Ok(())
    }
}
