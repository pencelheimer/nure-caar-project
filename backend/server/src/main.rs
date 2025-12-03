use std::{
    error::Error, //
    result::Result,
};

use sea_orm::Database;
use tokio::net::TcpListener;

use server::{
    config::Config, //
    controllers,
};

use tracing_subscriber::{
    layer::SubscriberExt, //
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().unwrap_or_default();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config: Config = Default::default();

    let _db = Database::connect(config.db_connection_str()).await?;

    let router = controllers::api_router();

    let listener = TcpListener::bind(config.socket()).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
