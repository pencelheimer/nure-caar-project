#[derive(Debug)]
pub struct Config {
    pub app_host: String,
    pub app_port: String,

    pub db_username: String,
    pub db_password: String,
    pub db_host: String,
    pub db_name: String,
}

impl Config {
    pub fn new() -> Self {
        let host = dotenvy::var("APP_HOST").unwrap_or("0.0.0.0".into());
        let port = dotenvy::var("APP_PORT").unwrap_or("3000".into());

        let db_username = dotenvy::var("DB_USERNAME").expect("DB_USERNAME env var is not present");
        let db_password = dotenvy::var("DB_PASSWORD").expect("DB_PASSWORD env var is not present");
        let db_host = dotenvy::var("DB_HOST").expect("DB_HOST env var is not present");
        let db_name = dotenvy::var("DB_NAME").expect("DB_NAME env var is not present");

        Config {
            app_host: host,
            app_port: port,
            db_username,
            db_password,
            db_host,
            db_name,
        }
    }

    pub fn socket(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }

    pub fn db_connection_str(&self) -> String {
        format!(
            "postgresql:///?user={}&password={}&dbname={}&host={}",
            self.db_username, self.db_password, self.db_name, self.db_host
        )
    }
}
