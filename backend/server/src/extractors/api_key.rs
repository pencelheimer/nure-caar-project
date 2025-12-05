use crate::{
    error::{
        AppError, //
        AuthError,
    },
    models::entities::{
        device, //
        prelude::Device,
        prelude::User,
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

        let result = Device::find()
            .filter(device::Column::ApiKey.eq(api_key))
            .find_also_related(User)
            .one(&state.db)
            .await?;

        let (device, user) = match result {
            Some((d, Some(u))) => (d, u),
            Some((_, None)) => return Err(AuthError::PermissionDenied)?,
            None => return Err(AuthError::InvalidToken)?,
        };

        if user.is_banned {
            return Err(AuthError::PermissionDenied.into());
        }

        Ok(AuthDevice {
            id: device.id,
            reservoir_id: device.reservoir_id,
        })
    }
}
