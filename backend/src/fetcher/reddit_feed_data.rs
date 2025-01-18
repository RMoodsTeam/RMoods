use crate::fetcher::data_request::{DataSource, FetcherDataRequest};
use crate::fetcher::fetcher_error::FetcherError;
use crate::fetcher::reddit::model::RawContainer;
use crate::fetcher::reddit::request::RedditRequest;
use std::fmt::Debug;

/// Describes a common interface for any data that is a feed in Reddit.
/// 1. Subreddit Posts
/// 2. Post Comments
/// 3. User Posts
pub trait FromRedditContainer: Send + Clone + Sized {
    /// Takes raw data form the underlying Reddit API connection and converts it into the high-level representation.
    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError>
    where
        Self: Sized;

    /// Concatenates two instances of the fetched feed data.
    /// This is used to merge the data fetched from multiple requests.
    fn concat(&mut self, other: Self) -> Self
    where
        Self: Sized;
}

/// Simpler trait for data that is fetched from the Reddit API as a single object, not as a feed.
pub trait RedditAboutData {
    type RequestType;
    /// Takes raw data form the underlying Reddit API connection and converts it into the high-level representation.
    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError>
    where
        Self: Sized;
}

pub trait RedditRequestable: FromRedditContainer {
    /// The type of the request that is used to fetch this data.
    type RequestType: RedditRequest + Debug;

    /// Creates a request to fetch the next page of data.
    fn create_reddit_request(
        request: &FetcherDataRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType
    where
        Self: RedditRequestable;
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
