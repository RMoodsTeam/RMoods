use crate::cast;
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::model::reddit_data::RedditAboutData;
use crate::reddit_fetcher::reddit::model::{RawContainer, RawUserAbout};
use crate::reddit_fetcher::reddit::request::UserAboutRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAbout {
    info: RawUserAbout,
}

impl RedditAboutData for UserAbout {
    type RequestType = UserAboutRequest;
    fn from_reddit_container(container: RawContainer) -> Result<UserAbout, FetcherError> {
        let info = cast!(container, RawContainer::UserAbout)?;

        Ok(UserAbout { info: *info })
    }
}
