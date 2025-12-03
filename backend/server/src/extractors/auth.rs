use crate::{
    error::{
        AppError, //
        AuthError,
    }, //
    models::{
        entities::sea_orm_active_enums::UserRole, //
        user::Users,
    },
    state::AppState,
    utils::jwt::Claims,
};

use axum::{
    RequestPartsExt, //
    extract::FromRequestParts,
    http::request::Parts,
};
use axum_extra::{
    TypedHeader, //
    headers::{
        Authorization, //
        authorization::Bearer,
    },
};
use jsonwebtoken::{
    DecodingKey, //
    Validation,
    decode,
    errors::ErrorKind,
};

pub struct AuthUser {
    pub id: i32,
    pub email: String,
    pub role: UserRole,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        let secret = &state.config.jwt_secret;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })?;

        let user_model = Users::find_by_id(&state.db, token_data.claims.id).await?;

        let user = user_model.ok_or(AuthError::UserNotFound)?;

        Ok(AuthUser {
            id: user.id,
            email: user.email,
            role: user.role,
        })
    }
}
