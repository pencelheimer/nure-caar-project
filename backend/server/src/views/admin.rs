use chrono::{
    DateTime, //
    FixedOffset,
};
use serde::{
    Deserialize, //
    Serialize,
};
use serde_json::Value;
use utoipa::{
    IntoParams, //
    ToSchema,
};

use crate::models::entities::sea_orm_active_enums::UserRole as DbUserRole;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    User,
    Viewer,
}

impl From<DbUserRole> for UserRole {
    fn from(val: DbUserRole) -> Self {
        match val {
            DbUserRole::Admin => Self::Admin,
            DbUserRole::User => Self::User,
            DbUserRole::Viewer => Self::Viewer,
        }
    }
}

impl From<UserRole> for DbUserRole {
    fn from(val: UserRole) -> Self {
        match val {
            UserRole::Admin => DbUserRole::Admin,
            UserRole::User => DbUserRole::User,
            UserRole::Viewer => DbUserRole::Viewer,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AdminUserDetailsResponse {
    pub id: i32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: UserRole,
    pub created_at: DateTime<FixedOffset>,
    pub reservoirs_count: u64,
    pub devices_count: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateRoleParams {
    pub role: UserRole,
}

#[derive(Deserialize, IntoParams)]
pub struct LogQuery {
    pub table_name: Option<String>,
    pub operation: Option<String>,
    pub record_id: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LogEntryResponse {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub operation: String,

    #[schema(value_type = Object)]
    pub old_values: Option<Value>,

    #[schema(value_type = Object)]
    pub new_values: Option<Value>,

    pub changed_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SystemStatsResponse {
    pub total_users: u64,
    pub total_reservoirs: u64,
    pub total_devices: u64,
    pub alert_rules_active: u64,
}
