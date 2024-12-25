use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Validation failed: {0}")]
    Invalid(String),
}
