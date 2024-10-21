#![allow(dead_code)]

use params::FeedSorting;

use super::model::MoreComments;
pub mod params;
mod tests;

/// Represents a request to the Reddit API.
///
/// Used to fetch posts, comments, and user/subreddit information.
/// The request is then converted into an HTTP request parts tuple.
///
/// # Example
///
/// ```
/// use params::{FeedSorting, FeedSortingTime};
///
/// let req = SubredditPosts {
///     subreddit: "Polska".to_string(),
///     sorting: FeedSorting::New
/// };

/// let (url, query) = req.into_http_request_parts();

/// assert_eq!(url, "https://oauth.reddit.com/r/Polska/new.json");
/// assert_eq!(query, vec![("limit", "100".to_string()), ("sort", "new".to_string())]);
/// ```
pub struct SubredditPostsRequest {
    /// The subreddit name.
    pub subreddit: String,
    pub sorting: FeedSorting,
    pub after: Option<String>,
}

/// Fetch information about a subreddit.
pub struct SubredditInfoRequest {
    pub subreddit: String,
}

/// Fetch posts from a user's profile.
pub struct UserPostsRequest {
    /// The user's username.
    pub username: String,
    pub sorting: FeedSorting,
    pub after: Option<String>,
}

/// Fetch information about a user.
pub struct UserInfoRequest {
    /// The user's username.
    pub username: String,
}

/// Fetch comments from a post.
pub struct PostCommentsRequest {
    /// The subreddit name.
    pub subreddit: String,
    /// The post's ID, eg. 1eubxgg
    pub post_id: String,
    pub sorting: FeedSorting,
    pub after: Option<String>,
}

/// The parts of an HTTP request: URL and query parameters.
pub type RequestParts = (String, Vec<(&'static str, String)>);

pub trait RedditResource {
    fn into_request_parts(&self) -> RequestParts;
}

impl RedditResource for SubredditPostsRequest {
    fn into_request_parts(&self) -> RequestParts {
        let url = format!(
            "https://oauth.reddit.com/r/{}/{}.json",
            self.subreddit,
            self.sorting.to_string()
        );

        let mut query = vec![];
        if let Some(time) = self.sorting.time() {
            query.push(("t", time.to_string()));
        }
        query.push(("limit", "100".to_string()));

        if let Some(after) = &self.after {
            query.push(("after", after.to_string()));
        }

        (url, query)
    }
}

impl RedditResource for SubredditInfoRequest {
    fn into_request_parts(&self) -> RequestParts {
        let url = format!("https://oauth.reddit.com/r/{}/about.json", self.subreddit);
        (url, vec![])
    }
}

impl RedditResource for UserPostsRequest {
    fn into_request_parts(&self) -> RequestParts {
        let url = format!("https://oauth.reddit.com/user/{}.json", self.username);

        let mut query = vec![("sort", self.sorting.to_string())];
        if let Some(time) = self.sorting.time() {
            query.push(("t", time.to_string()));
        }
        query.push(("limit", "100".to_string()));

        if let Some(after) = &self.after {
            query.push(("after", after.to_string()));
        }

        (url, query)
    }
}

impl RedditResource for UserInfoRequest {
    fn into_request_parts(&self) -> RequestParts {
        let url = format!("https://oauth.reddit.com/user/{}/about.json", self.username);
        (url, vec![])
    }
}

impl RedditResource for PostCommentsRequest {
    fn into_request_parts(&self) -> RequestParts {
        let url = format!(
            "https://oauth.reddit.com/r/{}/comments/{}.json",
            self.subreddit, self.post_id
        );

        let mut query = vec![("sort", self.sorting.to_string())];
        if let Some(time) = self.sorting.time() {
            query.push(("t", time.to_string()));
        }
        query.push(("limit", "100".to_string()));

        if let Some(after) = &self.after {
            query.push(("after", after.to_string()));
        }

        (url, query)
    }
}

impl MoreComments {
    pub fn into_request_parts(&self) -> Vec<RequestParts> {
        let url = format!("https://oauth.reddit.com/api/morechildren");

        self.children
            .chunks(100)
            .map(|chunk| {
                let query = vec![
                    ("link_id", self.parent_id.clone()),
                    ("children", chunk.join(",")),
                    ("api_type", "json".to_string()),
                ];

                (url.clone(), query)
            })
            .collect()
    }
}
