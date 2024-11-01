use crate::reddit_fetcher::feed_request::{DataSource, FetcherFeedRequest};
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::reddit::model::RawContainer;
use crate::reddit_fetcher::reddit::request::RedditRequest;

/// Describes a common interface for any data that is a feed in Reddit.
/// 1. Subreddit Posts
/// 2. Post Comments
/// 3. User Posts
pub trait RedditFeedData {
    /// The type of request that is used to fetch this data.
    type RequestType: RedditRequest;

    /// Takes raw data form the underlying Reddit API connection and converts it into the high-level representation.
    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError>
    where
        Self: Sized;
    /// Creates a request to fetch the next page of data.
    fn create_reddit_request(
        request: &FetcherFeedRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType;

    /// Concatenates two instances of the fetched feed data.
    /// This is used to merge the data fetched from multiple requests.
    fn concat(&mut self, other: Self) -> Self
    where
        Self: Sized;
}

/// Simpler trait for data that is fetched from the Reddit API as a single object, not as a feed.
pub trait RedditAboutData {
    /// The type of the request that is used to fetch this data.
    type RequestType: RedditRequest;

    /// Takes raw data form the underlying Reddit API connection and converts it into the high-level representation.
    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError>
    where
        Self: Sized;
}

/// Forcefully interpret a container variant as the chosen variant, else return an error
#[macro_export]
macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            Ok(a)
        } else {
            Err(FetcherError::RedditParseError(format!(
                "Failed to cast to {}",
                stringify!($pat)
            )))
        }
    }};
}
