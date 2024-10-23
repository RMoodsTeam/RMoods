use crate::cast;
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::model::reddit_data::RedditData;
use crate::reddit_fetcher::nlp_request::{DataSource, RMoodsNlpRequest};
use crate::reddit_fetcher::reddit::model::{RawComment, RawContainer, RawPost};
use crate::reddit_fetcher::reddit::request::UserPostsRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPosts {
    pub posts: Vec<RawPost>,
    pub comments: Vec<RawComment>,
}

impl RedditData for UserPosts {
    type RequestType = UserPostsRequest;
    fn from_reddit_container(container: RawContainer) -> Result<UserPosts, FetcherError> {
        let mut posts: Vec<RawPost> = Vec::new();
        let mut comments: Vec<RawComment> = Vec::new();

        let listing = cast!(container, RawContainer::Listing)?;

        for child in listing.children {
            match child {
                RawContainer::Post(post) => posts.push(*post),
                RawContainer::Comment(comment) => comments.push(*comment),
                _ => {
                    return Err(FetcherError::RedditParseError(
                        "Failed to parse post from Reddit container".to_string(),
                    )
                    .into());
                }
            }
        }

        Ok(UserPosts { posts, comments })
    }
    fn create_reddit_request(
        request: &RMoodsNlpRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType {
        UserPostsRequest {
            username: source.name,
            sorting: request.sorting,
            after,
        }
    }
    fn concat(&mut self, other: Self) -> Self {
        Self {
            posts: [self.posts.clone(), other.posts].concat(),
            comments: [self.comments.clone(), other.comments].concat(),
        }
    }
}
