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

    #[error("Failed to read credentials file. It should be placed in the same dir as Cargo.toml")]
    CredentialsNotFound,

    #[error("Malformed .reddit-credentials.json file. Please make sure it's valid JSON")]
    MalformedCredentials,

    #[error("Credentials file contains no credentials in the array")]
    NoClientsInCredentials,

    #[error("HTTP Error: `{0}`")]
    HttpError(#[from] reqwest::Error),

    #[error("Credentials not found: `{0}`")]
    InitError(#[from] std::io::Error),

    #[error("JSON Error: `{0}`")]
    JsonError(#[from] serde_json::Error),
}
