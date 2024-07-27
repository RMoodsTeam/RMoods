#![allow(dead_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedditError {
    #[error("Resource not found: '{0}'")]
    ResourceNotFound(String),

    #[error("Failed to fetch Reddit access token for client_id '{0}'")]
    FailedToFetchAccessToken(String),

    #[error("Domain '{0}' is not a valid Reddit API domain")]
    InvalidDomain(String),

    #[error("HTTP Error: `{0}`")]
    HttpError(#[from] reqwest::Error),

    #[error("Credentials not found: `{0}`")]
    InitError(#[from] std::io::Error),

    #[error("JSON Error: `{0}`")]
    JsonError(#[from] serde_json::Error),
}
