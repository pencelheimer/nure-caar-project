use server::{
    config::Config, //
    controllers,
    error::AppError,
    state::AppState,
};

use std::result::Result;
use sea_orm::Database;
use tokio::net::TcpListener;
use tracing_subscriber::{
    layer::SubscriberExt, //
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().unwrap_or_default();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config: Config = Default::default();
    let db = Database::connect(config.db_connection_str()).await?;
    let state = AppState {
        db,
        config: config.clone(),
    };

    let router = controllers::api_router();
    let router = router.with_state(state);

    let listener = TcpListener::bind(config.socket()).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
