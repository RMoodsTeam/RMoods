use crate::reddit::connection::RedditConnection;

/// Enum representing a request to the Reddit API
/// We can request information about a subreddit, posts from a subreddit, posts from a user, or information about a user
#[derive(Debug)]
pub enum RedditRequest {
    SubredditPosts(String),
    SubredditInfo(String),
    UserPosts(String),
    UserInfo(String),
    PostComments { subreddit: String, post_id: String },
}

impl RedditRequest {
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
