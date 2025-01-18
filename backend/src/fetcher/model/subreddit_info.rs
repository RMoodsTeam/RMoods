use crate::cast;
use crate::fetcher::fetcher_error::FetcherError;
use crate::fetcher::reddit::model::{RawContainer, RawSubredditAbout};
use crate::fetcher::reddit::request::SubredditAboutRequest;
use crate::fetcher::reddit_feed_data::RedditAboutData;
use serde::{Deserialize, Serialize};

/// Contains information about a subreddit.
/// It's a wrapper around the raw data returned by the Reddit API, just for consistency
#[derive(Debug, Serialize, Deserialize)]
pub struct SubredditAbout {
    pub info: RawSubredditAbout,
}

impl RedditAboutData for SubredditAbout {
    type RequestType = SubredditAboutRequest;
    fn from_reddit_container(container: RawContainer) -> Result<SubredditAbout, FetcherError> {
        let info = cast!(container, RawContainer::SubredditAbout)?;

        Ok(SubredditAbout { info: *info })
    }
}
