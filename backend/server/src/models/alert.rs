use crate::{
    error::{
        AppError, //
        AuthError,
        ResourceError,
    },
    models::entities::{
        alert, //
        alert_rule,
        prelude::*,
        reservoir,
        sea_orm_active_enums::{
            AlertConditionType, //
            AlertStatus,
        },
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
    ) -> Result<Vec<alert::Model>, AppError> {
        let alerts = Alert::find()
            .join(JoinType::InnerJoin, alert::Relation::AlertRule.def())
            .join(JoinType::InnerJoin, alert_rule::Relation::Reservoir.def())
            .filter(reservoir::Column::UserId.eq(user_id))
            .order_by_desc(alert::Column::TriggeredAt)
            .limit(params.limit.unwrap_or(50))
            .offset(params.offset.unwrap_or(0))
            .all(db)
            .await?;

        Ok(alerts)
    }

    pub async fn check_and_notify(
        db: &DatabaseConnection,
        reservoir_id: i32,
        value: f64,
    ) -> Result<(), AppError> {
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
                let message = format!(
                    "Alert for {}: Value {:.2} is {:?} threshold {:.2}",
                    reservoir.name, value, rule.condition_type, rule.threshold
                );

                let send_result = NotificationService::send_email(
                    &user_email,
                    "SmartTank Alert Triggered",
                    &message,
                )
                .await;

                let status = match send_result {
                    Ok(_) => AlertStatus::Sent,
                    Err(e) => {
                        tracing::error!("Failed to send alert email: {:?}", e);
                        AlertStatus::Failed
                    }
                };

                let alert_log = alert::ActiveModel {
                    rule_id: Set(rule.id),
                    sent_to: Set(user_email.clone()),
                    status: Set(status),
                    triggered_at: Set(Utc::now().into()),
                    ..Default::default()
                };

                alert_log.insert(db).await?;
            }
        }

        Ok(())
    }
}
