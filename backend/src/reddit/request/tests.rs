#[cfg(test)]
mod tests {
    use crate::reddit::request::{
        params::{FeedSorting, FeedSortingTime},
        RedditRequest,
    };

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_create_url_subreddit_posts() {
        let req = RedditRequest::SubredditPosts {
            subreddit: "Polska".to_string(),
            sorting: FeedSorting::New,
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(url, "https://oauth.reddit.com/r/Polska/new.json");
        assert_eq!(query, vec![("limit", "100".to_string())]);
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
            sorting: FeedSorting::Top(FeedSortingTime::All),
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(url, "https://oauth.reddit.com/user/spez.json");
        assert_eq!(
            query,
            vec![
                ("sort", "top".to_string()),
                ("t", "all".to_string()),
                ("limit", "100".to_string()),
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
            sorting: FeedSorting::Controversial(FeedSortingTime::Day),
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(
            url,
            "https://oauth.reddit.com/r/Polska/comments/abc123.json"
        );
        assert_eq!(
            query,
            vec![
                ("sort", "controversial".to_string()),
                ("t", "day".to_string()),
                ("limit", "100".to_string())
            ]
        );
    }

    #[test]
    fn test_create_default_params() {
        let req = RedditRequest::PostComments {
            subreddit: "Polska".to_string(),
            post_id: "abc123".to_string(),
            sorting: FeedSorting::default(),
        };
        let (url, query) = req.into_http_request_parts();
        assert_eq!(
            url,
            "https://oauth.reddit.com/r/Polska/comments/abc123.json"
        );
        assert_eq!(
            query,
            vec![("sort", "hot".to_string()), ("limit", "100".to_string())]
        );
    }
}
