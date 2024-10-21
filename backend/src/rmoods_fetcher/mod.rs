use log::debug;
use rmoods_request::{DataSource, RMoodsNlpRequest};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::reddit::{
    model::{MoreComments, RawComment, RawContainer, RawPost},
    request::{PostCommentsRequest, RedditResource, SubredditPostsRequest, UserPostsRequest},
};

pub mod rmoods_request;

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

#[derive(Debug, Error, Clone)]
pub enum FetcherError {
    #[error("Failed to parse data from Reddit: {0}")]
    RedditParseError(String),
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Posts {
    pub list: Vec<RawPost>,
}

impl RedditData for Posts {
    fn from_reddit_container(container: RawContainer) -> Result<Posts, FetcherError> {
        let mut posts: Vec<RawPost> = Vec::new();

        let listing = cast!(container, RawContainer::Listing)?;

        for child in listing.children {
            let post = cast!(child, RawContainer::Post)?;
            posts.push(*post);
        }

        Ok(Posts { list: posts })
    }
    type RequestType = SubredditPostsRequest;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPosts {
    pub posts: Vec<RawPost>,
    pub comments: Vec<RawComment>,
}

impl RedditData for UserPosts {
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
    type RequestType = UserPostsRequest;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PostComments {
    pub list: Vec<RawComment>,
    pub more: Vec<MoreComments>,
}

fn flatten_replies_internal(
    comment: &RawComment,
    all_replies: &mut Vec<RawComment>,
    mores: &mut Vec<MoreComments>,
    depth: u16,
) -> Result<(), FetcherError> {
    if let Some(replies_container) = comment.replies() {
        let listing = cast!(replies_container, RawContainer::Listing)?;

        let mut replies = vec![];
        for raw_reply in listing.children.clone() {
            // replies.push(cast!(raw_reply, RawContainer::Comment)?);
            match raw_reply {
                RawContainer::Comment(reply) => replies.push(*reply),
                RawContainer::More(more) => mores.push(*more),
                _ => {
                    return Err(FetcherError::RedditParseError(
                        "Failed to parse comment from Reddit container".to_string(),
                    )
                    .into());
                }
            }
        }

        let tabs = "  ".repeat((depth + 1).into());

        for mut reply in replies {
            debug!("{tabs}u/{}", reply.author());
            let _ = flatten_replies_internal(&reply, all_replies, mores, depth + 1)?;
            reply.replies = None;
            all_replies.push(reply.clone());
        }
    }
    Ok(())
}

fn flattened_replies(
    comment: &RawComment,
) -> Result<(Vec<RawComment>, Vec<MoreComments>), FetcherError> {
    let mut all_replies = vec![];
    let mut mores = vec![];

    let _ = flatten_replies_internal(comment, &mut all_replies, &mut mores, 0)?;

    Ok((all_replies, mores))
}

impl RedditData for PostComments {
    fn from_reddit_container(container: RawContainer) -> Result<PostComments, FetcherError> {
        let mut comments: Vec<RawComment> = Vec::new();

        let listing = cast!(container, RawContainer::Listing)?;
        let mut mores = vec![];

        for child in listing.children {
            // let mut comment = cast!(child, RawContainer::Comment)?;
            match child {
                RawContainer::Comment(mut comment) => {
                    let (mut replies, mut mores_2) = flattened_replies(&comment)?;
                    mores.append(&mut mores_2);
                    comments.append(&mut replies);

                    comment.replies = None;
                    comments.push(*comment);
                }
                RawContainer::More(more) => {
                    mores.push(*more)
                }
                _ => {
                    return Err(FetcherError::RedditParseError(
                        "Failed to parse comment from Reddit container".to_string(),
                    )
                    .into());
                }
            }
        }

        debug!("Returning {} post replies", { comments.len() });

        Ok(PostComments {
            list: comments,
            more: mores,
        })
    }
    type RequestType = PostCommentsRequest;
    fn create_reddit_request(
        request: &RMoodsNlpRequest,
        source: DataSource,
        after: Option<String>,
    ) -> Self::RequestType {
        PostCommentsRequest {
            subreddit: source.name,
            post_id: source.post_id.expect("Must be here"),
            sorting: request.sorting,
            after,
        }
    }
    fn concat(&mut self, other: Self) -> Self {
        Self {
            list: [self.list.clone(), other.list].concat(),
            more: [self.more.clone(), other.more].concat(),
        }
    }
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
