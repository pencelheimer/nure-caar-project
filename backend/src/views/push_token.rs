use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterPushTokenRequest {
    pub token: String,
    #[serde(default = "default_platform")]
    pub platform: String,
}

fn default_platform() -> String {
    "android".to_string()
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeletePushTokenRequest {
    pub token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PushTokenResponse {
    pub registered: bool,
}
