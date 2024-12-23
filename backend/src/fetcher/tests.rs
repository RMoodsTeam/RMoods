#[cfg(test)]
mod tests {
    use crate::fetcher::fetcher::RMoodsFetcher;
    use crate::fetcher::model::subreddit_info::SubredditAbout;
    use crate::fetcher::model::user_info::UserAbout;
    use crate::fetcher::reddit::request::{SubredditAboutRequest, UserAboutRequest};
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

    use once_cell::sync::OnceCell;
    use tokio::sync::Mutex;

    static HTTP: OnceCell<Client> = OnceCell::new();
    static FETCHER: OnceCell<Mutex<RMoodsFetcher>> = OnceCell::new();

    fn get_http() -> &'static Client {
        HTTP.get_or_init(|| ClientBuilder::new().user_agent("RMoods").build().unwrap())
    }

    async fn get_fetcher() -> &'static Mutex<RMoodsFetcher> {
        if let Some(fetcher) = FETCHER.get() {
            return fetcher;
        }

        let http = get_http();
        let fetcher = Mutex::new(RMoodsFetcher::new(http.clone()).await.unwrap());
        FETCHER.set(fetcher).unwrap();
        FETCHER.get().unwrap()
    }

    #[tokio::test]
    async fn fetch_about_user() {
        let fetcher = get_fetcher().await;
        let mut fetcher_lock = fetcher.lock().await;

        let request = UserAboutRequest {
            username: "spez".to_string(),
        };
        let user = fetcher_lock
            .fetch_about::<UserAbout>(request)
            .await
            .unwrap();
        assert_eq!(user.info.name, "spez");
    }

    #[tokio::test]
    async fn fetch_about_nonexistent_user() {
        let fetcher = get_fetcher().await;
        let mut fetcher_lock = fetcher.lock().await;
        let request = UserAboutRequest {
            username: random_string(20),
        };
        let user = fetcher.fetch_about::<UserAbout>(request).await;
        assert!(user.is_err());
    }

    #[tokio::test]
    async fn fetch_about_subreddit() {
        let fetcher = get_fetcher().await;
        let mut fetcher_lock = fetcher.lock().await;
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
        let fetcher = get_fetcher().await;
        let mut fetcher_lock = fetcher.lock().await;
        let request = SubredditAboutRequest {
            subreddit: random_string(20),
        };
        let sub = fetcher.fetch_about::<SubredditAbout>(request).await;
        assert!(sub.is_err());
    }
    // TODO: Feed fetching tests
}
