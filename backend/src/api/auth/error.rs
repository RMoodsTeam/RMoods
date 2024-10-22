#![allow(dead_code)]

use thiserror::Error;

/// Represents any kind of error that can occur when authenticating or authorizing users.
#[derive(Error, Debug)]
pub enum AuthError {
    /// Something went wrong with the JWT.
    ///
    /// See [jsonwebtoken::errors::ErrorKind](https://docs.rs/jsonwebtoken/7.0.1/jsonwebtoken/errors/enum.ErrorKind.html) for more information.
    #[error("JWT Error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    /// A request to Google failed
    #[error("Google Error: {0}")]
    GoogleError(#[from] reqwest::Error),
}
