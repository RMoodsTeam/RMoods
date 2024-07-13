use axum::{response::IntoResponse, Json};
use derive_getters::Getters;
use reqwest::StatusCode;
use serde_json::json;

use crate::reddit::reddit_error::RedditError;

#[derive(Debug, Getters)]
pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        AppError {
            code,
            message: message.into(),
        }
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
            RedditError::ResourceNotFound(res) => AppError::new(StatusCode::NOT_FOUND, value.to_string()),
            _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
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

    #[test]
    fn test_app_error_from_reddit_error_internal_server_error() {
        let error = RedditError::InvalidDomain("Invalid domain".to_string());
        let app_error: AppError = error.into();
        assert_eq!(app_error.code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(app_error.message, "Internal Server Error");
    }
}
