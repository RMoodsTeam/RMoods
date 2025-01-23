use crate::fetcher::model::post_comments::PostComments;
use crate::fetcher::model::posts::Posts;
use crate::fetcher::model::user_posts::UserPosts;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RedditDataContainer {
    SubredditPosts(Posts),
    PostComments(PostComments),
    UserPosts(UserPosts),
}

impl RedditDataContainer {
    pub fn values(&self) -> Result<Vec<Value>, serde_json::Error> {
        match self {
            RedditDataContainer::SubredditPosts(posts) => posts
                .list
                .iter()
                .map(|post| serde_json::to_value(post))
                .collect(),
            RedditDataContainer::PostComments(comments) => comments
                .list
                .iter()
                .map(|comment| serde_json::to_value(comment))
                .collect(),
            RedditDataContainer::UserPosts(user_posts) => user_posts
                .posts
                .iter()
                .map(|post| serde_json::to_value(post))
                .chain(
                    user_posts
                        .comments
                        .iter()
                        .map(|comment| serde_json::to_value(comment)),
                )
                .collect(),
        }
    }
}
