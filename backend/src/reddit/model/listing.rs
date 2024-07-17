use super::{comment::RedditComment, post::RedditPost};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "data")]
pub enum KindContainer {
    #[serde(rename = "Listing")]
    Listing(Box<RedditListing>), // Listing of something

    #[serde(rename = "more")]
    More(Box<MoreData>), // More

    #[serde(rename = "t1")]
    T1(Box<RedditComment>), // Comment

    #[serde(rename = "t2")]
    T2(Value), // Account TODO

    #[serde(rename = "t3")]
    T3(Box<RedditPost>), // Link/Post

    #[serde(rename = "t4")]
    T4(Value), // Message TODO

    #[serde(rename = "t5")]
    T5(Value), // Subreddit TODO

    #[serde(rename = "t6")]
    T6(Value), // Award TODO
}

/// List of IDs of items to fetch to get the ones that didn't fit in the first response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoreData {
    count: u32,
    name: String,
    id: String,
    parent_id: String,
    depth: u32,
    children: Vec<String>,
}

/// Describes a Reddit listing, which is a list of items with some metadata about the list and how to fetch more entries.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedditListing {
    after: Option<String>,
    dist: Option<u32>,
    children: Vec<KindContainer>,
    before: Option<String>,
}
