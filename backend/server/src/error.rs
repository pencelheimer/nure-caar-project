use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use error_set::error_set;
use serde_json::json;

error_set! {
    AppError := AuthError || SystemError

    AuthError := {
        #[display("Invalid token")]
        InvalidToken,

        #[display("Invalid credentials")]
        WrongCredentials,

        #[display("Token expired")]
        TokenExpired,

        #[display("Missing credentials")]
        MissingCredentials,

        #[display("User not found")]
        UserNotFound,

        #[display("User already exists")]
        UserAlreadyExists,

        #[display("Permission denied")]
        PermissionDenied,
    }

    SystemError := {
        #[display("Database error: {0}")]
        Database(sea_orm::DbErr),

        #[display("Password hashing error: {0}")]
        Hash(bcrypt::BcryptError),

        #[display("JWT error: {0}")]
        Jwt(jsonwebtoken::errors::Error),

        #[display("Database error: {0}")]
        Io(std::io::Error),

        #[display("Internal error: {message}")]
        Any { message: String },
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match &self {
            AppError::Database(_) | AppError::Hash(_) | AppError::Jwt(_) | AppError::Any { .. } => {
                tracing::error!(error = %self, "System error occurred")
            }
            _ => {} // NOTE(pencelheimer): No logs for logical errors
        }

        let (status, code) = match self {
            AppError::MissingCredentials => (StatusCode::UNAUTHORIZED, "MISSING_CREDENTIALS"),
            AppError::PermissionDenied => (StatusCode::FORBIDDEN, "PERMISSION_DENIED"),
            AppError::TokenExpired => (StatusCode::UNAUTHORIZED, "TOKEN_EXPIRED"),
            AppError::UserAlreadyExists => (StatusCode::BAD_REQUEST, "USER_EXISTS"),
            AppError::UserNotFound => (StatusCode::UNAUTHORIZED, "USER_NOT_FOUND"),
            AppError::WrongCredentials => (StatusCode::UNAUTHORIZED, "WRONG_CREDENTIALS"),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "INVALID_TOKEN"),

            AppError::Database(_) | AppError::Hash(_) | AppError::Jwt(_) | AppError::Io(_) | AppError::Any { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR")
            }
        };

        let message = if status == StatusCode::INTERNAL_SERVER_ERROR {
            "An internal server error occurred".to_string()
        } else {
            self.to_string()
        };

        let body = Json(json!({
            "error": {
                "code": code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}
