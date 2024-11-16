#[cfg(test)]
mod tests {
    use crate::reddit_fetcher::reddit::connection::RedditConnection;
    use lazy_static::lazy_static;
    use reqwest::{Client, ClientBuilder};

    lazy_static! {
        static ref HTTP: Client = ClientBuilder::new().user_agent("RMoods").build().unwrap();
    }
    static INIT: std::sync::Once = std::sync::Once::new();

    async fn init() -> RedditConnection {
        INIT.call_once(|| {
            let _ = dotenvy::dotenv();
        });
        RedditConnection::new(HTTP.clone()).await.unwrap()
    }

    #[tokio::test]
    async fn test_app_can_fetch_access_token() {
        let conn = init().await;
        conn.client.fetch_access_token(&conn.http).await.unwrap();
    }
}
