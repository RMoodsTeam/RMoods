use crate::reddit_fetcher::reddit::error::RedditError;
use thiserror::Error;

/// Describe the possible errors that can occur while fetching data from Reddit.
#[derive(Debug, Error)]
pub enum FetcherError {
    /// Error while parsing data from Reddit.
    #[error("Failed to parse data from Reddit: {0}")]
    RedditParseError(String),

    /// Error while fetching data from Reddit.
    /// This bubbles up from the underlying Reddit API client.
    #[error("Failed to fetch data from Reddit: {0}")]
    RedditApiError(#[from] RedditError),
}
