use crate::cast;
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::model::reddit_data::RedditAboutData;
use crate::reddit_fetcher::reddit::model::{RawContainer, RawSubredditAbout};
use crate::reddit_fetcher::reddit::request::SubredditAboutRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubredditAbout {
    info: RawSubredditAbout,
}

impl RedditAboutData for SubredditAbout {
    type RequestType = SubredditAboutRequest;
    fn from_reddit_container(container: RawContainer) -> Result<SubredditAbout, FetcherError> {
        let info = cast!(container, RawContainer::SubredditAbout)?;

        Ok(SubredditAbout { info: *info })
    }
}
