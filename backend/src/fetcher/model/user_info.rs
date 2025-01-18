use crate::cast;
use crate::fetcher::fetcher_error::FetcherError;
use crate::fetcher::reddit::model::{RawContainer, RawUserAbout};
use crate::fetcher::reddit::request::UserAboutRequest;
use crate::fetcher::reddit_feed_data::RedditAboutData;
use serde::{Deserialize, Serialize};

/// Contains information about a Reddit user.
/// It's a wrapper around the raw data returned by the Reddit API, just for consistency
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAbout {
    pub info: RawUserAbout,
}

impl RedditAboutData for UserAbout {
    type RequestType = UserAboutRequest;
    fn from_reddit_container(container: RawContainer) -> Result<UserAbout, FetcherError> {
        let info = cast!(container, RawContainer::UserAbout)?;

        Ok(UserAbout { info: *info })
    }
}
