use crate::validation::validation_error::ValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Database error: {0}")]
pub enum DbError {
    SqlxError(#[from] sqlx::Error),
    ValidationError(#[from] ValidationError),
    SerdeJsonError(#[from] serde_json::Error),
    NotFound(String),
}
