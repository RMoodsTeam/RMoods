use axum::{response::IntoResponse, Json};
use derive_getters::Getters;
use reqwest::StatusCode;
use serde_json::json;

use crate::api::auth::error::AuthError;
use crate::reddit_fetcher::reddit::error::RedditError;

/// Public-facing error kind. Contains an HTTP status code and a message describing the error.
#[derive(Debug, Getters)]
pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    /// Create a new AppError manually, with a given HTTP status code and error message.
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        AppError {
            code,
            message: message.into(),
        }
    }
    /// Shorthand for creating a 500 response.
    pub fn internal_server_error() -> Self {
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(json!({
                "status": self.code.to_string(),
                "message": self.message,
            })),
        )
            .into_response()
    }
}

impl From<RedditError> for AppError {
    fn from(value: RedditError) -> Self {
        match &value {
            RedditError::ResourceNotFound(_) => {
                AppError::new(StatusCode::NOT_FOUND, value.to_string())
            }
            _ => AppError::internal_server_error(),
        }
    }
}

impl From<AuthError> for AppError {
    fn from(value: AuthError) -> Self {
        type E = jsonwebtoken::errors::ErrorKind;
        match &value {
            AuthError::JwtError(e) => match e.kind() {
                E::ExpiredSignature => AppError::new(StatusCode::UNAUTHORIZED, "Expired token"),
                E::InvalidToken => AppError::new(StatusCode::UNAUTHORIZED, "Invalid token"),
                _ => AppError::internal_server_error(),
            },
            _ => AppError::internal_server_error(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    #[test]
    fn test_app_error_construction() {
        let error = AppError::new(StatusCode::NOT_FOUND, "Resource not found");
        assert_eq!(error.code, StatusCode::NOT_FOUND);
        assert_eq!(error.message, "Resource not found");
    }

    #[test]
    fn test_app_error_into_response() {
        let error = AppError::new(StatusCode::NOT_FOUND, "Resource not found");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_from_reddit_error_resource_not_found() {
        let error = RedditError::ResourceNotFound("r/Polska".to_string());
        let app_error: AppError = error.into();
        assert_eq!(app_error.code, StatusCode::NOT_FOUND);
        assert_eq!(app_error.message, "Resource not found: 'r/Polska'");
    }
}
