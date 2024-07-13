#![allow(dead_code)]
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Reddit subreddit data
/// https://www.reddit.com/r/Polska/about.json
#[derive(Getters, Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct RedditSubreddit {
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
    created_utc: u32,
    /// Fullname of the subreddit creator, eg. t2_1w72
    id: String,
    /// Language of the subreddit, eg. pl
    lang: String,
    /// Is the subreddit NSFW?
    /// This is inconsistent with the `over_18` field in RedditPost. THIS IS INTENTED AND CORRECT.
    over18: bool,
}
