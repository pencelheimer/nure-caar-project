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
