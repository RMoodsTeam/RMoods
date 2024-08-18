#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use reqwest::{Client, ClientBuilder};

    use crate::reddit::{connection::RedditConnection, request::RedditRequest};

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
}
