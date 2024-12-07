use axum::{response::IntoResponse, Json};
use derive_getters::Getters;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::json;

use crate::api::auth::error::AuthError;
use crate::nlp::error::NlpError;
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::reddit::error::RedditError;

/// Public-facing error kind. Contains an HTTP status code and a message describing the error.
#[derive(Debug, Getters, Clone, Serialize)]
pub struct AppError {
    #[serde(with = "http_serde::status_code")]
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

    /// Shorthand for creating a 404 response.
    pub fn not_found() -> Self {
        AppError::new(StatusCode::NOT_FOUND, "Resource not found")
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

/// Convert an AuthError into an AppError.
/// This is a public-facing error, so we return a 401 Unauthorized error and a short message.
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

/// Convert a FetcherError into an AppError.
/// Notify users about a case when a resource is not found.
/// In other cases, return a generic 500 Internal Server Error.
impl From<FetcherError> for AppError {
    fn from(_value: FetcherError) -> Self {
        match _value {
            FetcherError::RedditApiError(e) => match e {
                RedditError::ResourceNotFound(_) => {
                    AppError::new(StatusCode::NOT_FOUND, e.to_string())
                }
                _ => AppError::internal_server_error(),
            },
            _ => AppError::internal_server_error(),
        }
    }
}

impl From<NlpError> for AppError {
    fn from(_value: NlpError) -> Self {
        AppError::internal_server_error()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_value: sqlx::Error) -> Self {
        AppError::internal_server_error()
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
    fn test_reddit_not_found_error_into_app_error() {
        let reddit_err = RedditError::ResourceNotFound("Resource not found".to_string());
        let error = AppError::from(FetcherError::RedditApiError(reddit_err));
        assert_eq!(error.code, StatusCode::NOT_FOUND);
    }
}
