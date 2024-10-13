#![allow(dead_code)]

use params::FeedSorting;
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
/// let req = RedditRequest::SubredditPosts {
///     subreddit: "Polska".to_string(),
///     sorting: FeedSorting::New
/// };

/// let (url, query) = req.into_http_request_parts();

/// assert_eq!(url, "https://oauth.reddit.com/r/Polska/new.json");
/// assert_eq!(query, vec![("limit", "100".to_string()), ("sort", "new".to_string())]);
/// ```
///
#[derive(Debug)]
pub enum RedditRequest {
    /// Fetch posts from a subreddit.
    SubredditPosts {
        /// The subreddit name.
        subreddit: String,
        sorting: FeedSorting,
    },

    /// Fetch information about a subreddit.
    SubredditInfo { subreddit: String },

    /// Fetch posts from a user's profile.
    UserPosts {
        /// The user's username.
        username: String,
        sorting: FeedSorting,
    },

    /// Fetch information about a user.
    UserInfo {
        /// The user's username.
        username: String,
    },

    /// Fetch comments from a post.
    PostComments {
        /// The subreddit name.
        subreddit: String,
        /// The post's ID, eg. 1eubxgg
        post_id: String,
        sorting: FeedSorting,
    },
}

/// The parts of an HTTP request: URL and query parameters.
pub type RequestParts = (String, Vec<(&'static str, String)>);

impl RedditRequest {
    /// Check if the request is a feed request.
    /// Feed requests are requests that return a list of posts, not just information about a user or subreddit.
    fn is_feed_request(&self) -> bool {
        match self {
            RedditRequest::SubredditPosts { .. }
            | RedditRequest::UserPosts { .. }
            | RedditRequest::PostComments { .. } => true,
            _ => false,
        }
    }

    /// Convert the request into [RequestParts]
    ///
    /// Router method for converting any variant of a [RedditRequest] into [RequestParts].
    pub fn into_http_request_parts(self) -> RequestParts {
        type R = RedditRequest;

        let mut parts = match &self {
            R::SubredditPosts { subreddit, sorting } => {
                Self::subreddit_posts_to_http(&subreddit, sorting)
            }
            R::SubredditInfo { subreddit } => Self::subreddit_info_to_http(&subreddit),
            R::UserPosts { username, sorting } => Self::user_posts_to_http(&username, sorting),
            R::UserInfo { username } => Self::user_info_to_http(&username),
            R::PostComments {
                subreddit,
                post_id,
                sorting,
            } => Self::post_comments_to_http(&subreddit, &post_id, sorting),
        };

        // Only add the limit parameter if it's a feed request, it doesn't make sense for info requests.
        if self.is_feed_request() {
            parts.1.push(("limit", "100".to_string()));
        }

        parts
    }

    /// Convert [RedditRequest::SubredditPosts] into [RequestParts]
    fn subreddit_posts_to_http(subreddit: &str, sorting: &FeedSorting) -> RequestParts {
        // example: https://oauth.reddit.com/r/Polska/new.json?t=all
        // This is intentionally different from the other request types, that's how Reddit API works.
        let url = format!(
            "https://oauth.reddit.com/r/{}/{}.json",
            subreddit,
            sorting.to_string()
        );

        // Add the time if applicable
        let mut query = vec![];
        if let Some(time) = sorting.time() {
            query.push(("t", time.to_string()));
        }

        (url, query)
    }

    /// Convert [RedditRequest::SubredditInfo] into [RequestParts]
    fn subreddit_info_to_http(subreddit: &str) -> RequestParts {
        // example: https://oauth.reddit.com/r/Polska/about.json
        let url = format!("https://oauth.reddit.com/r/{}/about.json", subreddit);
        (url, vec![])
    }

    /// Convert [RedditRequest::UserPosts] into [RequestParts]
    fn user_posts_to_http(username: &str, sorting: &FeedSorting) -> RequestParts {
        // example: https://oauth.reddit.com/user/spez.json?sort=top&t=all
        let url = format!("https://oauth.reddit.com/user/{}.json", username);
        let mut query = vec![("sort", sorting.to_string())];
        if let Some(time) = sorting.time() {
            query.push(("t", time.to_string()));
        }
        (url, query)
    }

    /// Convert [RedditRequest::UserInfo] into [RequestParts]
    fn user_info_to_http(username: &str) -> RequestParts {
        // example: https://oauth.reddit.com/user/spez/about.json
        let url = format!("https://oauth.reddit.com/user/{}/about.json", username);
        (url, vec![])
    }

    /// Convert [RedditRequest::PostComments] into [RequestParts]
    fn post_comments_to_http(
        subreddit: &str,
        post_id: &str,
        sorting: &FeedSorting,
    ) -> RequestParts {
        // example: https://oauth.reddit.com/r/Polska/comments/abc123.json?sort=top&t=all
        let url = format!(
            "https://oauth.reddit.com/r/{}/comments/{}.json",
            subreddit, post_id
        );
        let mut query = vec![("sort", sorting.to_string())];
        if let Some(time) = sorting.time() {
            query.push(("t", time.to_string()));
        }
        (url, query)
    }
}
