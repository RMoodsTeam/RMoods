use crate::fetcher::fetcher_error::FetcherError;
use crate::nlp::error::NlpError;
use crate::validation::validation_error::ValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Failed to fetch Reddit data - {0}")]
    FetcherError(#[from] FetcherError),
    #[error("Failed to perform NLP analysis")]
    NlpError(#[from] NlpError),
    #[error("Failed to validate: {0}")]
    ValidationError(#[from] ValidationError),
}
