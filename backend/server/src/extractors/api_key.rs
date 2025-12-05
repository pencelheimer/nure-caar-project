use crate::{
    error::{
        AppError, //
        AuthError,
    },
    models::entities::{
        device, //
        prelude::Device,
    },
    state::AppState,
};

use axum::{
    extract::FromRequestParts, //
    http::request::Parts,
};
use sea_orm::{
    ColumnTrait, //
    EntityTrait,
    QueryFilter,
};

pub struct AuthDevice {
    pub id: i32,
    pub reservoir_id: Option<i32>,
}

impl FromRequestParts<AppState> for AuthDevice {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let api_key_header = parts
            .headers
            .get("x-api-key")
            .ok_or(AuthError::MissingCredentials)?;

        let api_key = api_key_header
            .to_str()
            .map_err(|_| AuthError::InvalidToken)?;

        let device_model = Device::find()
            .filter(device::Column::ApiKey.eq(api_key))
            .one(&state.db)
            .await?;

        let device = device_model.ok_or(AuthError::InvalidToken)?;

        Ok(AuthDevice {
            id: device.id,
            reservoir_id: device.reservoir_id,
        })
    }
}
