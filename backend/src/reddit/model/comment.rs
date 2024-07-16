use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::listing::KindContainer;

/// Empty string means no replies, otherwise it's a JSON array of RedditComment objects.
fn deserialize_replies<'de, D>(deserializer: D) -> Result<Option<KindContainer>, D::Error>
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

/// Reddit comment object
#[derive(Getters, Debug, Clone, Deserialize, Serialize)]
pub struct RedditComment {
    /// ID of the subreddit, eg. t5_2qh3s
    subreddit_id: String,
    /// Name of the subreddit, eg. Polska
    subreddit: String,
    /// Replies to the comment, if any
    #[serde(deserialize_with = "deserialize_replies")]
    replies: Option<KindContainer>,
    /// Comment author, eg. spez, without `u/`
    author: String,
    /// Comment text
    body: String,
    /// Standard url to the comment, without `.json` at the end
    permalink: String,
    /// UNIX timestamp of the comment creation
    created_utc: f32,
    /// Depth of the comment in the thread. 0 is the top-level comment, 1 is a reply to the top-level comment, etc.
    depth: u32,
    /// Upvotes - downvotes
    score: i64,
}
