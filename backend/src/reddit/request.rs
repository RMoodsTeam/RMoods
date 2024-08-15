use crate::reddit::connection::RedditConnection;

/// Enum representing a request to the Reddit API.
#[derive(Debug)]
pub enum RedditRequest {
    /// Get posts form a given subreddit, without their replies
    SubredditPosts(String),
    /// Get general information about a subreddit.
    SubredditInfo(String),
    /// Get posts from a given user, without replies.
    UserPosts(String),
    /// Get general information about a subreddit.
    UserInfo(String),
    /// Get the full post data and its reply tree.
    PostComments { subreddit: String, post_id: String },
}

impl RedditRequest {
    /// Get a URL that when fetched, will return what the [RedditRequest] wants to fetch.
    pub fn url(&self) -> String {
        let host = RedditConnection::API_HOSTNAME;
        type R = RedditRequest;
        match self {
            R::SubredditPosts(sub_name) => {
                format!("https://{}/r/{}/hot.json", host, sub_name)
            }
            R::SubredditInfo(sub_name) => {
                format!("https://{}/r/{}/about.json", host, sub_name)
            }
            R::UserPosts(username) => {
                format!("https://{}/user/{}.json", host, username)
            }
            R::UserInfo(username) => {
                format!("https://{}/user/{}/about.json", host, username)
            }
            R::PostComments { subreddit, post_id } => {
                format!("https://{}/r/{}/comments/{}.json", host, subreddit, post_id)
            }
        }
    }
}
