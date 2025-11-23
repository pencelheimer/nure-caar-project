use std::{error::Error, result::Result};

use axum::{Router, routing::get};
use server::config::Config;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    dotenvy::dotenv().unwrap_or_default();

    let config = Config::new();

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind(config.socket()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
