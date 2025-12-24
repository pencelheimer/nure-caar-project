use backend::{
    config::Config, //
    controllers,
    error::AppError,
    state::AppState,
};

use sea_orm::Database;
use std::result::Result;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{
    layer::SubscriberExt, //
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().unwrap_or_default();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config: Config = Default::default();
    let db = Database::connect(config.db_connection_str()).await?;
    let state = AppState {
        db,
        config: config.clone(),
    };

    let router = controllers::api_router()
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = config.socket();
    let listener = TcpListener::bind(addr.clone()).await?;
    tracing::info!("Server listening on {}", addr);
    axum::serve(listener, router).await?;

    Ok(())
}
