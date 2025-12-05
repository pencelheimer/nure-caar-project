use crate::views::admin::UserRole;

use chrono::{
    DateTime, //
    FixedOffset,
};
use serde::{
    Deserialize, //
    Serialize,
};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
    #[schema(example = "Jhon")]
    pub first_name: Option<String>,
    #[schema(example = "Doe")]
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserProfileResponse {
    pub id: i32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: UserRole,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateProfileRequest {
    #[schema(example = "Ivan")]
    pub first_name: Option<String>,
    #[schema(example = "Franko")]
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChangePasswordRequest {
    #[schema(example = "old_password123")]
    pub current_password: String,
    #[schema(example = "new_secure_password456")]
    pub new_password: String,
}
