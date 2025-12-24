use crate::models::entities::{
    alert_rule::Model as DbAlertRule, //
    alert::Model as DbAlertLog,
    sea_orm_active_enums::{
        AlertConditionType as DbAlertConditionType, //
        AlertStatus as DbAlertStatus,
    },
};

use chrono::{
    DateTime, //
    FixedOffset,
};
use serde::{
    Deserialize, //
    Serialize,
};
use utoipa::{
    IntoParams, //
    ToSchema,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlertConditionType {
    GreaterThan,
    LessThan,
    Equals,
}

impl From<DbAlertConditionType> for AlertConditionType {
    fn from(val: DbAlertConditionType) -> Self {
        match val {
            DbAlertConditionType::GreaterThan => Self::GreaterThan,
            DbAlertConditionType::LessThan => Self::LessThan,
            DbAlertConditionType::Equals => Self::Equals,
        }
    }
}

impl From<AlertConditionType> for DbAlertConditionType {
    fn from(val: AlertConditionType) -> Self {
        match val {
            AlertConditionType::GreaterThan => Self::GreaterThan,
            AlertConditionType::LessThan => Self::LessThan,
            AlertConditionType::Equals => Self::Equals,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    Pending,
    Sent,
    Failed,
}

impl From<DbAlertStatus> for AlertStatus {
    fn from(val: DbAlertStatus) -> Self {
        match val {
            DbAlertStatus::Pending => Self::Pending,
            DbAlertStatus::Sent => Self::Sent,
            DbAlertStatus::Failed => Self::Failed,
        }
    }
}

impl From<AlertStatus> for DbAlertStatus {
    fn from(val: AlertStatus) -> Self {
        match val {
            AlertStatus::Pending => Self::Pending,
            AlertStatus::Sent => Self::Sent,
            AlertStatus::Failed => Self::Failed,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateAlertRuleRequest {
    pub reservoir_id: i32,
    #[schema(example = "less_than")]
    pub condition_type: AlertConditionType,
    #[schema(example = 200.0)]
    pub threshold: f64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AlertRuleResponse {
    pub id: i32,
    pub reservoir_id: i32,
    pub condition_type: AlertConditionType,
    pub threshold: f64,
    pub is_active: bool,
}

impl From<DbAlertRule> for AlertRuleResponse {
    fn from(val: DbAlertRule) -> Self {
        Self {
            id: val.id,
            reservoir_id: val.reservoir_id,
            condition_type: val.condition_type.into(),
            threshold: val.threshold,
            is_active: val.is_active.unwrap(), // populated by the DB
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AlertLogResponse {
    pub id: i32,
    pub rule_id: i32,
    pub triggered_at: DateTime<FixedOffset>,
    pub sent_to: String,
    pub status: AlertStatus,
}

impl From<DbAlertLog> for AlertLogResponse {
    fn from(val: DbAlertLog) -> Self {
        Self {
            id: val.id,
            rule_id: val.rule_id,
            triggered_at: val.triggered_at,
            sent_to: val.sent_to,
            status: val.status.into(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateAlertRuleRequest {
    pub condition_type: Option<AlertConditionType>,
    pub threshold: Option<f64>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize, IntoParams)]
pub struct AlertHistoryQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
