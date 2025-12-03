use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    User,
    Viewer,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AdminUserDetailsResponse {
    pub id: i32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: UserRole,
    pub created_at: DateTime<FixedOffset>,
    pub reservoirs_count: i64,
    pub devices_count: i64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRoleRequest {
    pub role: UserRole,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BanUserRequest {
    pub is_banned: bool,
    pub ban_reason: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SystemConfigResponse {
    pub maintenance_mode: bool,
    pub registration_enabled: bool,
    pub default_data_retention_days: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateSystemConfigRequest {
    pub maintenance_mode: Option<bool>,
    pub registration_enabled: Option<bool>,
    pub default_data_retention_days: Option<i32>,
}

#[derive(Deserialize, IntoParams)]
pub struct LogQuery {
    pub level: Option<String>,
    pub user_id: Option<i32>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LogEntryResponse {
    pub id: String,
    pub timestamp: DateTime<FixedOffset>,
    pub level: String,
    pub message: String,
    pub user_email: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SystemStatsResponse {
    pub total_users: i64,
    pub total_reservoirs: i64,
    pub total_devices: i64,
    pub online_devices: i64,
    pub alerts_today: i64,
}
