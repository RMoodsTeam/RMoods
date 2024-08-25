#[cfg(test)]
mod tests {
    use crate::reddit::request::{
        params::{FeedRequestParams, FeedSorting, FeedSortingTime, RequestSize},
        RedditRequest,
    };

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_create_url_subreddit_posts() {
        let req = RedditRequest::SubredditPosts {
            subreddit: "Polska".to_string(),
            params: FeedRequestParams {
                size: RequestSize::Medium,
                sorting: FeedSorting::New,
            },
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(url, "https://oauth.reddit.com/r/Polska/new.json");
        assert_eq!(query, vec![("limit", "250".to_string())]);
    }

    #[test]
    fn test_create_url_subreddit_info() {
        let req = RedditRequest::SubredditInfo {
            subreddit: "Polska".to_string(),
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(url, "https://oauth.reddit.com/r/Polska/about.json");
        assert_eq!(query, vec![]);
    }

    #[test]
    fn test_create_url_user_posts() {
        let req = RedditRequest::UserPosts {
            username: "spez".to_string(),
            params: FeedRequestParams {
                size: RequestSize::Small,
                sorting: FeedSorting::Top(FeedSortingTime::All),
            },
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(url, "https://oauth.reddit.com/user/spez.json");
        assert_eq!(
            query,
            vec![
                ("limit", "50".to_string()),
                ("sort", "top".to_string()),
                ("t", "all".to_string())
            ]
        );
    }

    #[test]
    fn test_create_url_user_info() {
        let req = RedditRequest::UserInfo {
            username: "spez".to_string(),
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(url, "https://oauth.reddit.com/user/spez/about.json");
        assert_eq!(query, vec![]);
    }

    #[test]
    fn test_create_url_post_comments() {
        let req = RedditRequest::PostComments {
            subreddit: "Polska".to_string(),
            post_id: "abc123".to_string(),
            params: FeedRequestParams {
                size: RequestSize::Custom(10),
                sorting: FeedSorting::Controversial(FeedSortingTime::Day),
            },
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(
            url,
            "https://oauth.reddit.com/r/Polska/comments/abc123.json"
        );
        assert_eq!(
            query,
            vec![
                ("limit", "10".to_string()),
                ("sort", "controversial".to_string()),
                ("t", "day".to_string())
            ]
        );
    }

    #[test]
    fn test_create_default_params() {
        let req = RedditRequest::PostComments {
            subreddit: "Polska".to_string(),
            post_id: "abc123".to_string(),
            params: FeedRequestParams::default(),
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(
            url,
            "https://oauth.reddit.com/r/Polska/comments/abc123.json"
        );
        assert_eq!(
            query,
            vec![("limit", "50".to_string()), ("sort", "hot".to_string())]
        );
    }
}
