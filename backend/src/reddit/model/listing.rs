use super::{comment::Comment, post::Post, subreddit_info::SubredditInfo, user::UserInfo};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
pub enum KindContainer {
    #[serde(rename = "Listing")]
    Listing(Box<RedditListing>), // Listing of something

    #[serde(rename = "more")]
    More(Box<MoreData>), // More

    #[serde(rename = "t1")]
    T1(Box<Comment>), // Comment

    #[serde(rename = "t2")]
    T2(Box<UserInfo>), // Account TODO

    #[serde(rename = "t3")]
    T3(Box<Post>), // Link/Post

    #[serde(rename = "t4")]
    T4(Box<Value>), // Message TODO

    #[serde(rename = "t5")]
    T5(Box<SubredditInfo>), // Subreddit Info

    #[serde(rename = "t6")]
    T6(Box<Value>), // Award TODO
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

/// Describes a Reddit listing
/// 
/// It's a list of items with some metadata about the list and how to fetch more entries.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedditListing {
    after: Option<String>,
    dist: Option<u32>,
    children: Vec<KindContainer>,
    before: Option<String>,
}
