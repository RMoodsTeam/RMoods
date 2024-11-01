use crate::cast;
use crate::reddit_fetcher::feed_request::{DataSource, FetcherFeedRequest};
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::model::reddit_data::RedditFeedData;
use crate::reddit_fetcher::reddit::model::{RawContainer, RawPost};
use crate::reddit_fetcher::reddit::request::SubredditPostsRequest;
use log_derive::logfn;
use serde::{Deserialize, Serialize};

/// Contains the posts of a subreddit.
/// Posts are to be fetches by using the `Fetcher::fetch_feed` method with appropriate parameters.
#[derive(Debug, Serialize, Deserialize)]
pub struct Posts {
    pub list: Vec<RawPost>,
}

impl RedditFeedData for Posts {
    type RequestType = SubredditPostsRequest;

    #[logfn(err = "ERROR", fmt = "Failed to parse from RedditContainer: {0}")]
    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError> {
        let mut posts: Vec<RawPost> = Vec::new();

        let listing = cast!(container, RawContainer::Listing)?;

        for child in listing.children {
            let post = cast!(child, RawContainer::Post)?;
            posts.push(*post);
        }

        Ok(Self { list: posts })
    }
    fn create_reddit_request(
        request: &FetcherFeedRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType {
        Self::RequestType {
            subreddit: source.name,
            sorting: request.sorting,
            after,
        }
    }
    fn concat(&mut self, other: Self) -> Self {
        Self {
            list: [self.list.clone(), other.list].concat(),
        }
    }
}
