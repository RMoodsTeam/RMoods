use crate::cast;
use crate::fetcher::data_request::{DataSource, FetcherDataRequest};
use crate::fetcher::fetcher_error::FetcherError;
use crate::fetcher::reddit::model::{RawComment, RawContainer, RawPost};
use crate::fetcher::reddit::request::UserPostsRequest;
use crate::fetcher::reddit_feed_data::{FromRedditContainer, RedditRequestable};
use log_derive::logfn;
use serde::Serialize;

/// Contains the posts and comments of a Reddit user.
/// Posts and comments are to be fetches by using the `Fetcher::fetch_feed` method with appropriate parameters.
/// The user's feed contains both posts and comments, so this struct contains both.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct UserPosts {
    pub posts: Vec<RawPost>,
    pub comments: Vec<RawComment>,
}

impl FromRedditContainer for UserPosts {
    #[logfn(err = "ERROR", fmt = "Failed to parse from RedditContainer: {0}")]
    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError> {
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

        Ok(Self { posts, comments })
    }

    fn concat(&mut self, other: Self) -> Self {
        Self {
            posts: [self.posts.clone(), other.posts].concat(),
            comments: [self.comments.clone(), other.comments].concat(),
        }
    }
}

impl RedditRequestable for UserPosts {
    type RequestType = UserPostsRequest;

    fn create_reddit_request(
        request: &FetcherDataRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType {
        Self::RequestType {
            username: source.name,
            sorting: request.sort_by,
            after,
        }
    }
}

pub enum PostOrComment {
    Post(RawPost),
    Comment(RawComment),
}

impl IntoIterator for UserPosts {
    type Item = PostOrComment;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let posts = self.posts.into_iter().map(PostOrComment::Post);
        let comments = self.comments.into_iter().map(PostOrComment::Comment);
        posts
            .chain(comments)
            .collect::<Vec<PostOrComment>>()
            .into_iter()
    }
}
