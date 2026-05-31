#[derive(Debug, Clone)]
pub struct Config {
    pub app_host: String,
    pub app_port: String,

    pub db_url: String,

    pub jwt_secret: String,

    /// JSON contents of a Firebase service account key file.
    /// When set, FCM push notifications are enabled.
    /// Leave empty to run without push support.
    pub firebase_service_account_json: String,
}

impl Config {
    pub fn new() -> Self {
        let app_host = dotenvy::var("APP_HOST").unwrap_or("0.0.0.0".into());
        let app_port = dotenvy::var("APP_PORT").unwrap_or("6969".into());

        let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL env var is not present");

        let jwt_secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET env var is not present");

        let firebase_service_account_json =
            dotenvy::var("FIREBASE_SERVICE_ACCOUNT_JSON").unwrap_or_default();

        Config {
            app_host,
            app_port,
            db_url,
            jwt_secret,
            firebase_service_account_json,
        }
    }

    pub fn socket(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }

    pub fn db_connection_str(&self) -> String {
        self.db_url.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
