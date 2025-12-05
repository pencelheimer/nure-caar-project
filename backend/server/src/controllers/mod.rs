mod admin;
mod auth;
mod device;
mod health;
mod measurement;
mod reservoir;

use utoipa::{
    Modify, //
    OpenApi,
    openapi::security::{
        ApiKey, //
        ApiKeyValue,
        Http,
        HttpAuthScheme,
        SecurityScheme,
    },
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{
    Scalar, //
    Servable,
};

use crate::state::AppState;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-api-key"))),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(title = "SmartTank", description = "SmartTank OpenAPI specification", version = "0.1.0",)
)]
struct ApiSpec;

pub fn api_router() -> axum::Router<AppState> {
    let (router, api) = OpenApiRouter::with_openapi(ApiSpec::openapi())
        .merge(health::register_routes())
        .merge(admin::register_routes())
        .merge(auth::register_routes())
        .merge(device::register_routes())
        .merge(measurement::register_routes())
        .merge(reservoir::register_routes())
        .split_for_parts();

    router.merge(Scalar::with_url("/", api))
}
