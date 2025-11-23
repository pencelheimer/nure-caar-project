use std::{
    error::Error,   //
    result::Result, //
};

use axum::Router;
use sea_orm::Database;
use tokio::net::TcpListener;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_scalar::{
    Scalar,   //
    Servable, //
};

use server::config::Config;
use server::controllers::*;
use migrations::{
    Migrator,      //
    MigratorTrait, //
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    dotenvy::dotenv().unwrap_or_default();

    let config = Config::new();
    let db = Database::connect(config.db_connection_str()).await?;

    Migrator::fresh(&db).await?;

    let (app, api): (Router, OpenApi) = OpenApiRouter::new()
        .routes(routes!(hello_world))
        .split_for_parts();

    let app = app.merge(Scalar::with_url("/scalar", api));

    let listener = TcpListener::bind(config.socket()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
