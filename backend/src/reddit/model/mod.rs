#![allow(dead_code)]

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, NoneAsEmptyString};

/// Represents a generic Reddit data container.
///
/// Reddit API uses a special structure to represent varying types of object in a uniform way.
/// This struct is actually the heart of the Reddit response parsing.
///
/// ### How does it work?
/// It's kind of a declarative way to parse the response. The [KindContainer] struct described the universal structure of every Reddit response.
/// Reddit uses the following structure to let us know what type of data to expect:
/// ```json
/// {
///     "kind" : "t1",
///     "data" : {
///         <Comment data>
///     }
/// }
/// ```
/// So, we have a `kind` field and the corresponding `data` field, which has an object of the type indicated by `kind`.
///
/// [Serde](https://serde.rs) provides a convenient macro, that lets us describe how our enum should be handled in serialization/deserialization:
/// ```rust
/// #[serde(tag = "kind", content = "data")]
/// enum Foo {}
/// ```
/// When used like in the example above, this macro lets us represent our exact structure: object tagged by `kind` with the content in `data`.
/// Content is the value embedded in the enum variant, eg. `Box<RedditListing>`.
///
/// Each field uses the `#[serde(rename = "foo")]` macro, to tell [Serde](https://serde.rs) to look for that given name during its operations.
/// This paired with the previously described tagging, lets us use the [KindContainer] struct to recrusively, automatically and reliably parse all Reddit responses.
///
/// ### Example
/// Probably not accurate execution order, written just to understand the process.
/// 1. We tell serde to deserialize some JSON data into KindContainer.
/// 2. Serde looks for the `kind` field.
/// 3. Let's assume `kind: "t1"`.
/// 4. Since `kind` is `"t1"`, serde knows that the enum variant it encountered is KindContainer::T1(Box<Comment>)
/// 5. Serde finds the `data` field and treats it as the content of type Box<Comment>
/// 6. [Box] is transparent to serde, so it parses the `data` field as a [Comment](super::comment:Comment)
///
/// ### Recursive case
/// For the [KindContainer::Listing], this works recursively, since [RedditListing] is basically a list of [KindContainer]s.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "data")]
pub enum RawContainer {
    #[serde(rename = "Listing")]
    Listing(Box<RawListing>),

    #[serde(rename = "more")]
    More(Box<MoreComments>),

    #[serde(rename = "t1")]
    Comment(Box<RawComment>),

    #[serde(rename = "t2")]
    UserInfo(Box<RawUserInfo>),

    #[serde(rename = "t3")]
    Post(Box<RawPost>), // Link/Post

    #[serde(rename = "t4")]
    Message(Box<Value>), // Message TODO

    #[serde(rename = "t5")]
    SubredditInfo(Box<RawSubredditInfo>),

    #[serde(rename = "t6")]
    Award(Box<Value>), // Award TODO
}

/// List of IDs of items to fetch to get the ones that didn't fit in the first response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoreComments {
    pub count: u32,
    pub name: String,
    pub id: String,
    pub parent_id: String,
    pub depth: u32,
    pub children: Vec<String>,
}

/// Describes a Reddit listing
///
/// It's a list of items with some metadata about the list and how to fetch more entries.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawListing {
    pub after: Option<String>,
    pub dist: Option<u32>,
    pub children: Vec<RawContainer>,
    pub before: Option<String>,
}

/// Represents a reply to a [Post](super::post::Post)
#[derive(Getters, Debug, Clone, Deserialize, Serialize)]
pub struct RawComment {
    /// ID of the subreddit, eg. t5_2qh3s
    subreddit_id: String,
    /// Name of the subreddit, eg. Polska
    subreddit: String,
    /// Replies to the comment, if any
    #[serde(deserialize_with = "deserialize_replies")]
    pub replies: Option<RawContainer>,
    /// Comment author, eg. spez, without `u/`
    author: String,
    /// Comment text
    body: String,
    /// Standard url to the comment, without `.json` at the end
    permalink: String,
    /// UNIX timestamp of the comment creation
    created_utc: f32,
    /// Depth of the comment in the thread. 0 is the top-level comment, 1 is a reply to the top-level comment, etc.
    ///
    /// When fetching posts/comments from a user, this field is always `None`.
    depth: Option<u32>,
    /// Upvotes - downvotes
    score: i64,
}

/// Contains some properties of a Reddit post. For some real-world examples see
/// [this r/Polska request.](https://www.reddit.com/r/Polska.json)
#[derive(Getters, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RawPost {
    /// eg. Polska
    subreddit: String,
    /// Text of the post
    selftext: String,
    /// Number of golds
    gilded: u32,
    /// Post title
    title: String,
    /// Fullname, eg. t3_8z1v1z
    name: String,
    /// Upvotes - downvotes
    score: i64,
    /// UNIX timestamp of the post creation
    created_utc: f32,
    /// Is the post NSFW?
    /// This is inconsistent with the `over18` field in Subreddit. THIS IS INTENTED AND CORRECT.
    over_18: bool,
    /// Fullname without the kind info, eg. 8z1v
    id: String,
    /// Subreddit fullname, eg. t5_2qh3s
    subreddit_id: String,
    /// username without `u/` eg. spez
    author: String,
    /// Number of comments
    num_comments: u32,
    /// Standard url, without `.json` at the end
    url: String,
    /// Is the post stickied?
    stickied: bool,
}

/// Reddit subreddit data
/// https://www.reddit.com/r/Polska/about.json
#[derive(Getters, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RawSubredditInfo {
    /// eg. Polska
    display_name: String,
    /// tagline, eg. "Polski Subreddit"
    title: String,
    /// Primary color of the subreddit, eg. #0079d3
    primary_color: String,
    /// Secondary color of the subreddit, eg. #ff4500
    key_color: String,
    /// Number of active users
    active_user_count: u32,
    /// Number of subscribers
    subscribers: u32,
    /// Fullname, eg. t5_2qh3s
    name: String,
    /// Description of the subreddit
    public_description: String,
    /// Subreddit logo, link needs to be parsed to get the actual image
    community_icon: String,
    /// Banner background image, link needs to be parsed to get the actual image
    banner_background_image: String,
    /// UNIX timestamp of the subreddit creation
    created_utc: f32,
    /// Fullname of the subreddit creator, eg. t2_1w72
    id: String,
    /// Language of the subreddit, eg. pl
    lang: String,
    /// Is the subreddit NSFW?
    /// This is inconsistent with the `over_18` field in RedditPost. THIS IS INTENTED AND CORRECT.
    over18: bool,
}

/// Contains some properties of a Reddit user. For some real-world examples see
/// [this u/spez profile request.](https://www.reddit.com/user/spez/about.json)
#[serde_as]
#[derive(Getters, Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct RawUserInfo {
    /// Is the user a Reddit employee?
    is_employee: bool,
    /// Karma received by getting awards
    awardee_karma: i64,
    /// User ID, eg. 1w72
    id: String,
    /// Is the user a verified?
    verified: bool,
    /// Karma given to others by giving awards
    awarder_karma: i64,
    /// Better described as `post karma`, received for others upvoting your posts
    link_karma: i64,
    /// Karma received for others upvoting your comments
    comment_karma: i64,
    /// Total karma, sum of all karma types. Maybe remove this field and calculate it?
    total_karma: i64,
    /// Username without `u/`, eg. spez
    name: String,
    /// Link to the user icon
    #[serde_as(as = "NoneAsEmptyString")]
    icon_img: Option<String>,
    /// Link to the user snoovatar, whatever that is
    #[serde_as(as = "NoneAsEmptyString")]
    snoovatar_img: Option<String>,
}

/// Empty string means no replies, otherwise it's a JSON array of RedditComment objects.
fn deserialize_replies<'de, D>(deserializer: D) -> Result<Option<RawContainer>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let replies = Value::deserialize(deserializer)?;

    // When there are no replies, Reddit returns an empty string, not a JSON null,
    // but we want to check just in case. Else, it's a KindContainer with RedditComment objects,
    // but that's not important at this moment, we just treat it a value.
    match &replies {
        Value::Null | Value::String(_) => Ok(None),
        _ => Ok(serde_json::from_value(replies).map_err(serde::de::Error::custom)?),
    }
}
