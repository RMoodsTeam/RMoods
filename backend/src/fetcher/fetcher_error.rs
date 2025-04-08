use crate::fetcher::reddit::error::RedditError;
use crate::validation::validation_error::ValidationError;
use thiserror::Error;

/// Describe the possible errors that can occur while fetching data from Reddit.
#[derive(Debug, Error)]
pub enum FetcherError {
    /// Error while parsing data from Reddit.
    #[error("Failed to parse data from Reddit: {0}")]
    RedditParseError(String),

    /// Error while fetching data from Reddit.
    /// This bubbles up from the underlying Reddit API client.
    #[error(transparent)]
    RedditApiError(#[from] RedditError),

    /// Invalid request
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}
