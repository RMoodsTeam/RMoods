#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use reqwest::{Client, ClientBuilder};
    
    use crate::reddit::{RedditConnection, RedditRequest};

    lazy_static! {
        static ref HTTP: Client = ClientBuilder::new().user_agent("RMoods").build().unwrap();
    }
    static INIT: std::sync::Once = std::sync::Once::new();
    
    fn init() {
        INIT.call_once(|| {
            let _ = dotenvy::dotenv();
        })
    }

    #[tokio::test]
    async fn test_app_can_fetch_access_token() {
        init();
        let conn = RedditConnection::new(HTTP.clone()).await.unwrap();
        let _ = conn.client.fetch_access_token(&conn.http).await;
    }

    #[test]
    fn test_create_url_subreddit_posts() {
        let url = RedditRequest::SubredditPosts("Polska".to_string()).url();
        assert_eq!(url, "https://oauth.reddit.com/r/Polska/hot.json");
    }

    #[test]
    fn test_create_url_subreddit_info() {
        let url = RedditRequest::SubredditInfo("Polska".to_string()).url();
        assert_eq!(url, "https://oauth.reddit.com/r/Polska/about.json");
    }

    #[test]
    fn test_create_url_user_posts() {
        let url = RedditRequest::UserPosts("spez".to_string()).url();
        assert_eq!(url, "https://oauth.reddit.com/user/spez.json");
    }

    #[test]
    fn test_create_url_user_info() {
        let url = RedditRequest::UserInfo("spez".to_string()).url();
        assert_eq!(url, "https://oauth.reddit.com/user/spez/about.json");
    }

    #[tokio::test]
    async fn test_fetch_subreddit_posts() {
        init();
        let mut conn = RedditConnection::new(HTTP.clone()).await.unwrap();
        let req = RedditRequest::SubredditPosts("Polska".to_string());
        let _ = conn.fetch_raw(req).await.unwrap();
    }

    #[tokio::test]
    async fn test_fetch_subreddit_info() {
        init();
        let mut conn = RedditConnection::new(HTTP.clone()).await.unwrap();
        let req = RedditRequest::SubredditInfo("Polska".to_string());
        let _ = conn.fetch_raw(req).await.unwrap();
    }

    #[tokio::test]
    async fn test_fetch_user_posts() {
        init();
        let mut conn = RedditConnection::new(HTTP.clone()).await.unwrap();
        let req = RedditRequest::UserPosts("spez".to_string());
        let _ = conn.fetch_raw(req).await.unwrap();
    }

    #[tokio::test]
    async fn test_fetch_user_info() {
        init();
        let mut conn = RedditConnection::new(HTTP.clone()).await.unwrap();
        let req = RedditRequest::UserInfo("spez".to_string());
        let _ = conn.fetch_raw(req).await.unwrap();
    }
}
