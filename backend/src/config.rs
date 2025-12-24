#[derive(Debug, Clone)]
pub struct Config {
    pub app_host: String,
    pub app_port: String,

    pub db_url: String,

    pub jwt_secret: String,
}

impl Config {
    pub fn new() -> Self {
        let app_host = dotenvy::var("APP_HOST").unwrap_or("0.0.0.0".into());
        let app_port = dotenvy::var("APP_PORT").unwrap_or("6969".into());

        let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL env var is not present");

        let jwt_secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET env var is not present");

        Config {
            app_host,
            app_port,
            db_url,
            jwt_secret,
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
