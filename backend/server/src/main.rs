use std::{
    error::Error,   //
    result::Result, //
};

use sea_orm::Database;
use tokio::net::TcpListener;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_scalar::{
    Scalar,   //
    Servable, //
};

use migrations::{
    Migrator,      //
    MigratorTrait, //
};
use server::config::Config;
use server::controllers::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().unwrap_or_default();
    env_logger::init();

    let config: Config = Default::default();

    let db = Database::connect(config.db_connection_str()).await?;
    Migrator::fresh(&db).await?;

    let router = {
        let (router, api) = OpenApiRouter::new()
            .routes(routes!(hello_world))
            .split_for_parts();

        router.merge(Scalar::with_url("/scalar", api))
    };

    let listener = TcpListener::bind(config.socket()).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
