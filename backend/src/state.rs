use sea_orm::DatabaseConnection;

use crate::{config::Config, services::fcm::FcmClient};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
    pub http: reqwest::Client,
    /// None when FIREBASE_SERVICE_ACCOUNT_JSON is not set.
    pub fcm: Option<FcmClient>,
}
