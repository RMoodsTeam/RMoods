#![allow(dead_code)]

use params::FeedRequestParams;
mod params;
mod tests;

/// Represents a request to the Reddit API.
///
/// Used to fetch posts, comments, and user/subreddit information.
/// The request is then converted into an HTTP request parts tuple.
///
/// # Example
///
/// ```
/// use params::{FeedRequestParams, FeedSorting, FeedSortingTime, RequestSize}};
///
/// let req = RedditRequest::SubredditPosts {
///     subreddit: "Polska".to_string(),
///     params: FeedRequestParams {
///         size: RequestSize::Medium,
///         sorting: FeedSorting::New,
///     },
/// };
/// let (url, query) = req.into_http_request_parts();
/// assert_eq!(url, "https://oauth.reddit.com/r/Polska/new.json");
/// assert_eq!(query, vec![("limit", "250".to_string()), ("sort", "new".to_string())]);
/// ```
///
#[derive(Debug)]
pub enum RedditRequest {
    /// Fetch posts from a subreddit.
    SubredditPosts {
        /// The subreddit name.
        subreddit: String,
        params: FeedRequestParams,
    },

    /// Fetch information about a subreddit.
    SubredditInfo { subreddit: String },

    /// Fetch posts from a user's profile.
    UserPosts {
        /// The user's username.
        username: String,
        params: FeedRequestParams,
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
        params: FeedRequestParams,
    },
}

/// The parts of an HTTP request: URL and query parameters.
pub type RequestParts = (String, Vec<(&'static str, String)>);

impl RedditRequest {
    /// Convert the request into [RequestParts]
    ///
    /// Router method for converting any variant of a [RedditRequest] into [RequestParts].
    pub fn into_http_request_parts(self) -> RequestParts {
        type R = RedditRequest;

        match self {
            R::SubredditPosts { subreddit, params } => {
                Self::subreddit_posts_to_http(&subreddit, params)
            }
            R::SubredditInfo { subreddit } => Self::subreddit_info_to_http(&subreddit),
            R::UserPosts { username, params } => Self::user_posts_to_http(&username, params),
            R::UserInfo { username } => Self::user_info_to_http(&username),
            R::PostComments {
                subreddit,
                post_id,
                params,
            } => Self::post_comments_to_http(&subreddit, &post_id, params),
        }
    }

    /// Convert [RedditRequest::SubredditPosts] into [RequestParts]
    fn subreddit_posts_to_http(subreddit: &str, params: FeedRequestParams) -> RequestParts {
        let url = format!(
            "https://oauth.reddit.com/r/{}/{}.json",
            subreddit,
            params.sorting().to_string()
        );
        let mut query = vec![("limit", params.size().to_string())];
        if let Some(time) = params.sorting().time() {
            query.push(("t", time.to_string()));
        }

        (url, query)
    }

    /// Convert [RedditRequest::SubredditInfo] into [RequestParts]
    fn subreddit_info_to_http(subreddit: &str) -> RequestParts {
        let url = format!("https://oauth.reddit.com/r/{}/about.json", subreddit);
        (url, vec![])
    }

    /// Convert [RedditRequest::UserPosts] into [RequestParts]
    fn user_posts_to_http(username: &str, params: FeedRequestParams) -> RequestParts {
        let url = format!("https://oauth.reddit.com/user/{}.json", username);
        let mut query = vec![
            ("limit", params.size().to_string()),
            ("sort", params.sorting().to_string()),
        ];
        if let Some(time) = params.sorting().time() {
            query.push(("t", time.to_string()));
        }
        (url, query)
    }

    /// Convert [RedditRequest::UserInfo] into [RequestParts]
    fn user_info_to_http(username: &str) -> RequestParts {
        let url = format!("https://oauth.reddit.com/user/{}/about.json", username);
        (url, vec![])
    }

    /// Convert [RedditRequest::PostComments] into [RequestParts]
    fn post_comments_to_http(
        subreddit: &str,
        post_id: &str,
        params: FeedRequestParams,
    ) -> RequestParts {
        let url = format!(
            "https://oauth.reddit.com/r/{}/comments/{}.json",
            subreddit, post_id
        );
        let mut query = vec![
            ("limit", params.size().to_string()),
            ("sort", params.sorting().to_string()),
        ];
        if let Some(time) = params.sorting().time() {
            query.push(("t", time.to_string()));
        }
        (url, query)
    }
}
