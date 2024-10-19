use log::debug;
use rmoods_request::RedditFeedKind;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::reddit::model::{RawComment, RawContainer, RawPost, RawSubredditInfo, RawUserInfo};

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
    fn from_reddit_container(container: RawContainer) -> Result<Self, FetcherError>
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostComments {
    pub list: Vec<RawComment>,
}

fn flatten_replies_internal(
    comment: &RawComment,
    all_replies: &mut Vec<RawComment>,
    depth: u16,
) -> Result<(), FetcherError> {
    if let Some(replies_container) = comment.replies() {
        let listing = cast!(replies_container, RawContainer::Listing)?;

        let mut replies = vec![];
        for x in listing.children.clone() {
            replies.push(cast!(x, RawContainer::Comment)?);
        }

        let tabs = "  ".repeat((depth + 1).into());

        for mut reply in replies {
            debug!("{tabs}u/{}", reply.author());
            let _ = flatten_replies_internal(&reply, all_replies, depth + 1)?;
            reply.replies = None;
            all_replies.push(*reply.clone());
        }
    }
    Ok(())
}

fn flattened_replies(comment: &RawComment) -> Result<Vec<RawComment>, FetcherError> {
    debug!("u/{}", comment.author());
    let mut all_replies = Vec::new();

    let _ = flatten_replies_internal(comment, &mut all_replies, 0)?;

    Ok(all_replies)
}

impl RedditData for PostComments {
    fn from_reddit_container(container: RawContainer) -> Result<PostComments, FetcherError> {
        let mut comments: Vec<RawComment> = Vec::new();

        let listing = cast!(container, RawContainer::Listing)?;

        for child in listing.children {
            let mut comment = cast!(child, RawContainer::Comment)?;
            let mut replies = flattened_replies(&comment)?;
            comments.append(&mut replies);

            comment.replies = None;
            comments.push(*comment);
        }

        debug!("Returning {} post replies", { comments.len() });

        Ok(PostComments { list: comments })
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
