use crate::cast;
use crate::fetcher::data_request::{DataSource, FetcherDataRequest};
use crate::fetcher::fetcher_error::FetcherError;
use crate::fetcher::model::reddit_data::RedditFeedData;
use crate::fetcher::reddit::model::{RawContainer, RawPost};
use crate::fetcher::reddit::request::SubredditPostsRequest;
use log_derive::logfn;
use serde::Serialize;

/// Contains the posts of a subreddit.
/// Posts are to be fetches by using the `Fetcher::fetch_feed` method with appropriate parameters.
///
// TODO: Make all similar structs private, unobtainable for the user
#[derive(Debug, Serialize, Clone)]
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
        request: &FetcherDataRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType {
        Self::RequestType {
            subreddit: source.name,
            sorting: request.sort_by,
            after,
        }
    }
    fn concat(&mut self, other: Self) -> Self {
        Self {
            list: [self.list.clone(), other.list].concat(),
        }
    }

    fn extract_texts(self) -> Vec<String> {
        self.list.into_iter().map(|post| post.title).collect()
    }
}
