use std::{
    error::Error,   //
    result::Result, //
};

use sea_orm::Database;
use tokio::net::TcpListener;

use migrations::{
    Migrator,      //
    MigratorTrait, //
};
use server::{
    config::Config, //
    controllers //
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().unwrap_or_default();
    env_logger::init();

    let config: Config = Default::default();

    let db = Database::connect(config.db_connection_str()).await?;
    Migrator::fresh(&db).await?;

    let router = controllers::api_router();

    let listener = TcpListener::bind(config.socket()).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
