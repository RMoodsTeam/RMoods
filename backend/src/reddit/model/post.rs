use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Contains some properties of a Reddit post. For some real-world examples see
/// [this r/Polska request.](https://www.reddit.com/r/Polska.json)
#[derive(Getters, Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct RedditPost {
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
    created_utc: u32,
    /// Is the post NSFW?
    /// This is inconsistent with the `over18` field in RedditSubreddit. THIS IS INTENTED AND CORRECT.
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
