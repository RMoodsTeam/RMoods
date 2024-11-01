use crate::reddit_fetcher::feed_request::{DataSource, FetcherFeedRequest};
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::reddit::model::RawContainer;
use crate::reddit_fetcher::reddit::request::RedditRequest;

pub trait RedditFeedData {
    type RequestType: RedditRequest;

    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError>
    where
        Self: Sized;
    fn create_reddit_request(
        request: &FetcherFeedRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType;
    fn concat(&mut self, other: Self) -> Self
    where
        Self: Sized;
}

pub trait RedditAboutData {
    type RequestType: RedditRequest;
    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError>
    where
        Self: Sized;
}

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
