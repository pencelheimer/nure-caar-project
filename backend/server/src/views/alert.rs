use serde::{
    Deserialize, //
    Serialize,
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlertConditionType {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
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
