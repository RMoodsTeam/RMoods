use derive_getters::Getters;
use serde::Deserialize;

/// Empty string means no replies, otherwise it's a JSON array of RedditComment objects.
fn deserialize_replies<'de, D>(deserializer: D) -> Result<Option<Vec<RedditComment>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let v = serde_json::from_str(&s).map_err(serde::de::Error::custom)?;
        Ok(Some(v))
    }
}

#[derive(Getters, Debug, Hash, Eq, PartialEq, Clone, Deserialize)]
pub struct RedditComment {
    /// ID of the subreddit, eg. t5_2qh3s
    subreddit_id: String,
    /// Name of the subreddit, eg. Polska
    subreddit: String,
    /// Replies to the comment, if any
    #[serde(deserialize_with = "deserialize_replies")]
    replies: Option<Vec<RedditComment>>,
    /// Comment author, eg. spez, without `u/`
    author: String,
    /// Comment text
    body: String,
    /// Standard url to the comment, without `.json` at the end
    permalink: String,
    /// UNIX timestamp of the comment creation
    created_utc: u32,
    /// Depth of the comment in the thread. 0 is the top-level comment, 1 is a reply to the top-level comment, etc.
    depth: u32,
    /// Upvotes - downvotes
    score: i64,
}
