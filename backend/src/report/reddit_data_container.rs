use crate::fetcher::model::post_comments::PostComments;
use crate::fetcher::model::posts::Posts;
use crate::fetcher::model::user_posts::UserPosts;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RedditDataContainer {
    SubredditPosts(Posts),
    PostComments(PostComments),
    UserPosts(UserPosts),
}
