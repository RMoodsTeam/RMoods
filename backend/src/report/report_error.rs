use crate::fetcher::fetcher_error::FetcherError;
use crate::nlp::error::NlpError;
use crate::validation::validation_error::ValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Error generating report: {0}")]
pub enum ReportError {
    #[error("Error fetching Reddit data: {0}")]
    FetcherError(#[from] FetcherError),
    #[error("Error during NLP analysis: {0}")]
    NlpError(#[from] NlpError),
    #[error("Error validating report: {0}")]
    ValidationError(#[from] ValidationError),
}
