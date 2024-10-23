use crate::cast;
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::model::reddit_data::RedditData;
use crate::reddit_fetcher::nlp_request::{DataSource, RMoodsNlpRequest};
use crate::reddit_fetcher::reddit::model::{RawContainer, RawPost};
use crate::reddit_fetcher::reddit::request::SubredditPostsRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Posts {
    pub list: Vec<RawPost>,
}

impl RedditData for Posts {
    type RequestType = SubredditPostsRequest;
    fn from_reddit_container(container: RawContainer) -> Result<Posts, FetcherError> {
        let mut posts: Vec<RawPost> = Vec::new();

        let listing = cast!(container, RawContainer::Listing)?;

        for child in listing.children {
            let post = cast!(child, RawContainer::Post)?;
            posts.push(*post);
        }

        Ok(Posts { list: posts })
    }
    fn create_reddit_request(
        request: &RMoodsNlpRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType {
        SubredditPostsRequest {
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
