use thiserror::Error;

/// An error that can occur when interacting with the NLP Service.
#[derive(Debug, Error)]
pub enum NlpError {
    /// Occurs when the NLP Service returns a response of unexpected format.
    #[error("Failed to deserialize NLP response: {0}")]
    NlpSerdeError(#[from] serde_json::Error),
    /// Occurs when communication with NLP Service has failed.
    #[error("Failed to communicate with NLP service: {0}")]
    NlpHttpError(#[from] reqwest::Error),
}
