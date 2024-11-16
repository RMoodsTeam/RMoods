#[cfg(test)]
mod tests {
    use crate::reddit_fetcher::fetcher::RMoodsFetcher;
    use crate::reddit_fetcher::model::subreddit_info::SubredditAbout;
    use crate::reddit_fetcher::model::user_info::UserAbout;
    use crate::reddit_fetcher::reddit::request::{SubredditAboutRequest, UserAboutRequest};
    use lazy_static::lazy_static;
    use reqwest::{Client, ClientBuilder};

    fn random_string(len: usize) -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    lazy_static! {
        static ref HTTP: Client = ClientBuilder::new().user_agent("RMoods").build().unwrap();
    }
    static INIT: std::sync::Once = std::sync::Once::new();

    async fn init() -> RMoodsFetcher {
        INIT.call_once(|| {
            let _ = dotenvy::dotenv();
        });
        RMoodsFetcher::new(HTTP.clone()).await.unwrap()
    }

    #[tokio::test]
    async fn fetch_about_user() {
        let mut fetcher = init().await;
        let request = UserAboutRequest {
            username: "spez".to_string(),
        };
        let user = fetcher.fetch_about::<UserAbout>(request).await.unwrap();
        assert_eq!(user.info.name, "spez");
    }

    #[tokio::test]
    async fn fetch_about_nonexistent_user() {
        let mut fetcher = init().await;
        let request = UserAboutRequest {
            username: random_string(20),
        };
        let user = fetcher.fetch_about::<UserAbout>(request).await;
        assert!(user.is_err());
    }

    #[tokio::test]
    async fn fetch_about_subreddit() {
        let mut fetcher = init().await;
        let request = SubredditAboutRequest {
            subreddit: "programming".to_string(),
        };
        let sub = fetcher
            .fetch_about::<SubredditAbout>(request)
            .await
            .unwrap();
        assert_eq!(sub.info.display_name, "programming");
    }

    #[tokio::test]
    async fn fetch_about_nonexistent_subreddit() {
        let mut fetcher = init().await;
        let request = SubredditAboutRequest {
            subreddit: random_string(20),
        };
        let sub = fetcher.fetch_about::<SubredditAbout>(request).await;
        assert!(sub.is_err());
    }
    // TODO: Feed fetching tests
}
