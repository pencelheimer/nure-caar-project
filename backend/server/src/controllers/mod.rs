mod auth;
mod device;
mod measurement;
mod reservoir;

use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{
    Scalar,   //
    Servable, //
};

#[derive(OpenApi)]
#[openapi(info(
    title = "SmartTank",
    description = "SmartTank OpenAPI specification",
    version = "0.1.0",
))]
struct ApiSpec;

pub fn api_router() -> axum::Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiSpec::openapi())
        .merge(auth::register_routes())
        .merge(device::register_routes())
        .merge(measurement::register_routes())
        .merge(reservoir::register_routes())
        .split_for_parts();

    router.merge(Scalar::with_url("/", api))
}
