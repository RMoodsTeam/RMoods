use crate::reddit::model::post::Post;
use crate::reddit::model::subreddit_info::SubredditInfo;
use crate::reddit::model::user::UserInfo;
use crate::reddit::model::{comment::Comment, listing::KindContainer};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use thiserror::Error;

macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            a
        } else {
            panic!("Mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

#[derive(Debug, Error)]
pub enum FetcherError {
    #[error("Failed to parse data from Reddit: {0}")]
    RedditParseError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Posts {
    pub list: Vec<Post>,
}

pub trait RedditData {
    fn from_reddit_container(container: KindContainer) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl RedditData for Posts {
    fn from_reddit_container(container: KindContainer) -> anyhow::Result<Self> {
        let mut posts: Vec<Post> = Vec::new();

        let listing = cast!(container, KindContainer::Listing);

        for child in listing.children {
            let post = cast!(child, KindContainer::Post);
            posts.push(*post);
        }

        Ok(Posts { list: posts })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPosts {
    pub posts: Vec<Post>,
    pub comments: Vec<Comment>,
}

impl RedditData for UserPosts {
    fn from_reddit_container(container: KindContainer) -> anyhow::Result<Self> {
        let mut posts: Vec<Post> = Vec::new();
        let mut comments: Vec<Comment> = Vec::new();

        let listing = cast!(container, KindContainer::Listing);

        for child in listing.children {
            match child {
                KindContainer::Post(post) => posts.push(*post),
                KindContainer::Comment(comment) => comments.push(*comment),
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
    pub list: Vec<Comment>,
}

fn flatten_replies_internal(comment: &Comment, all_replies: &mut Vec<Comment>, depth: u16) {
    if let Some(replies_container) = comment.replies() {
        let listing = cast!(replies_container, KindContainer::Listing);

        let mut replies = vec![];
        for x in listing.children.clone() {
            replies.push(cast!(x, KindContainer::Comment));
        }

        let tabs = "  ".repeat((depth + 1).into());

        for mut reply in replies {
            debug!("{tabs}u/{}", reply.author());
            flatten_replies_internal(&reply, all_replies, depth + 1);
            reply.replies = None;
            all_replies.push(*reply.clone());
        }
    }
}

fn flattened_replies(comment: &Comment) -> Vec<Comment> {
    debug!("u/{}", comment.author());
    let mut all_replies = Vec::new();

    flatten_replies_internal(comment, &mut all_replies, 0);

    all_replies
}

impl RedditData for PostComments {
    fn from_reddit_container(container: KindContainer) -> anyhow::Result<Self> {
        let mut comments: Vec<Comment> = Vec::new();

        let listing = cast!(container, KindContainer::Listing);

        for child in listing.children {
            let mut comment = cast!(child, KindContainer::Comment);
            let mut replies = flattened_replies(&comment);
            comments.append(&mut replies);

            comment.replies = None;
            comments.push(*comment);
        }

        debug!("Returning {} post replies", { comments.len() });

        Ok(PostComments { list: comments })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedUserInfo {
    info: UserInfo,
}

impl RedditData for ParsedUserInfo {
    fn from_reddit_container(container: KindContainer) -> anyhow::Result<Self> {
        let info = cast!(container, KindContainer::UserInfo);

        Ok(ParsedUserInfo { info: *info })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedSubredditInfo {
    info: SubredditInfo,
}

impl RedditData for ParsedSubredditInfo {
    fn from_reddit_container(container: KindContainer) -> anyhow::Result<Self> {
        let info = cast!(container, KindContainer::SubredditInfo);

        Ok(ParsedSubredditInfo { info: *info })
    }
}
