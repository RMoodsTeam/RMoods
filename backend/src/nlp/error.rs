use thiserror::Error;

#[derive(Debug, Error)]
pub enum NlpError {
    #[error("Failed to deserialize NLP response: {0}")]
    NlpSerdeError(#[from] serde_json::Error),
    #[error("Failed to communicate with NLP service: {0}")]
    NlpHttpError(#[from] reqwest::Error),
}
