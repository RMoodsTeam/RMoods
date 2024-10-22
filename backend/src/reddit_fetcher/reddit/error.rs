use thiserror::Error;

/// Represents any kind of error that can occur when dealing with the Reddit API.
#[derive(Error, Debug)]
pub enum RedditError {
    /// We request data for a non-existent subreddit or user etc.
    #[error("Resource not found: '{0}'")]
    ResourceNotFound(String),

    /// The request that fetches the Reddit access token for our app failed.
    #[error("Failed to fetch Reddit access token for client_id '{0}'")]
    FailedToFetchAccessToken(String),

    /// Some other HTTP error occurred.
    #[error("HTTP Error: `{0}`")]
    HttpError(#[from] reqwest::Error),

    /// Deserialization to our structs failed.
    ///
    /// Possible causes:
    /// * Our struct definitions are wrong
    /// * We didn't check for an error status, and we're trying to parse an error response to our structs
    /// * Reddit sent a malformed response (incredibly unlikely)
    #[error("JSON Error: `{0}`")]
    JsonError(#[from] serde_json::Error),

    #[error("Failed to parse data from Reddit: {0}")]
    OtherJsonError(String),
}
