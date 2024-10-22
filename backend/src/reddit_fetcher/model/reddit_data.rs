use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::nlp_request::{DataSource, RMoodsNlpRequest};
use crate::reddit_fetcher::reddit::model::RawContainer;
use crate::reddit_fetcher::reddit::request::RedditResource;

pub trait RedditData {
    type RequestType: RedditResource;
    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError>
    where
        Self: Sized;
    fn create_reddit_request(
        request: &RMoodsNlpRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType;
    fn concat(&mut self, other: Self) -> Self
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

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserInfo {
//     info: RawUserInfo,
// }

// impl RedditData for UserInfo {
//     fn from_reddit_container(container: RawContainer) -> Result<UserInfo, FetcherError> {
//         let info = cast!(container, RawContainer::UserInfo)?;

//         Ok(UserInfo { info: *info })
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SubredditInfo {
//     info: RawSubredditInfo,
// }

// impl RedditData for SubredditInfo {
//     fn from_reddit_container(container: RawContainer) -> Result<SubredditInfo, FetcherError> {
//         let info = cast!(container, RawContainer::SubredditInfo)?;

//         Ok(SubredditInfo { info: *info })
//     }
// }
