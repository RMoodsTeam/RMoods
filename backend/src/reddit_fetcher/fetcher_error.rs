use crate::reddit_fetcher::reddit::error::RedditError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FetcherError {
    #[error("Failed to parse data from Reddit: {0}")]
    RedditParseError(String),

    #[error("Failed to fetch data from Reddit: {0}")]
    RedditApiError(#[from] RedditError),
}
