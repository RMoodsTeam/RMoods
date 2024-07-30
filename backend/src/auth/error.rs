#![allow(dead_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("JWT Error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Google Error: {0}")]
    GoogleError(#[from] reqwest::Error),
}
