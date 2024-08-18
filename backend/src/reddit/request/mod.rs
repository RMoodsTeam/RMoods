#![allow(dead_code)]

use params::FeedRequestParams;
mod params;
mod tests;

#[derive(Debug)]
pub enum RedditRequest {
    SubredditPosts {
        subreddit: String,
        params: FeedRequestParams,
    },

    SubredditInfo {
        subreddit: String,
    },

    UserPosts {
        username: String,
        params: FeedRequestParams,
    },

    UserInfo {
        username: String,
    },

    PostComments {
        subreddit: String,
        post_id: String,
        params: FeedRequestParams,
    },
}

pub type RequestParts = (String, Vec<(&'static str, String)>);

impl RedditRequest {
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

    fn subreddit_info_to_http(subreddit: &str) -> RequestParts {
        let url = format!("https://oauth.reddit.com/r/{}/about.json", subreddit);
        (url, vec![])
    }

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

    fn user_info_to_http(username: &str) -> RequestParts {
        let url = format!("https://oauth.reddit.com/user/{}/about.json", username);
        (url, vec![])
    }

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
