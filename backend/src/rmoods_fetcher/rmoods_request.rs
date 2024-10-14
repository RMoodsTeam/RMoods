use crate::reddit::{connection::RedditConnection, model::RawContainer, request::RedditRequest};

use super::{PostComments, RedditData, SubredditInfo, UserInfo, UserPosts};

pub enum RMoodsRequestKind {
    SubredditInfo,
    UserPosts,
    UserInfo,
    PostComments,
}

pub struct RMoodsRequest {
    kind: RMoodsRequestKind,
    size: RequestSize,
}

pub enum RequestSize {
    Small,
    Medium,
    Large,
    Custom(u16),
}

impl Default for RequestSize {
    fn default() -> Self {
        RequestSize::Medium
    }
}

impl From<RequestSize> for u16 {
    fn from(value: RequestSize) -> Self {
        match value {
            RequestSize::Small => 50,
            RequestSize::Medium => 250,
            RequestSize::Large => 500,
            RequestSize::Custom(n) => n,
        }
    }
}

pub struct RMoodsFetcher {
    reddit_connection: RedditConnection,
}

impl RMoodsFetcher {
    pub fn new(reddit_connection: RedditConnection) -> Self {
        RMoodsFetcher { reddit_connection }
    }

    pub fn fetch(request: RMoodsRequest) -> anyhow::Result<impl RedditData> {}
}
