use crate::fetcher::model::post_comments::PostComments;
use crate::fetcher::model::posts::Posts;
use crate::fetcher::model::user_posts::UserPosts;
use crate::fetcher::reddit::model::{RawComment, RawPost};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RedditDataContainer {
    SubredditPosts(Posts),
    PostComments(PostComments),
    UserPosts(UserPosts),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RedditItem {
    Post(RawPost),
    Comment(RawComment),
}

impl RedditDataContainer {
    pub fn items(&self) -> Vec<RedditItem> {
        match self {
            RedditDataContainer::SubredditPosts(posts) => posts
                .list
                .iter()
                .map(|post| RedditItem::Post(post.clone()))
                .collect(),
            RedditDataContainer::PostComments(comments) => comments
                .list
                .iter()
                .map(|comment| RedditItem::Comment(comment.clone()))
                .collect(),
            RedditDataContainer::UserPosts(user_posts) => user_posts
                .posts
                .iter()
                .map(|post| RedditItem::Post(post.clone()))
                .chain(
                    user_posts
                        .comments
                        .iter()
                        .map(|comment| RedditItem::Comment(comment.clone())),
                )
                .collect(),
        }
    }
}
