#[cfg(test)]
mod tests {
    use crate::fetcher::fetcher::RMoodsFetcher;
    use crate::fetcher::model::subreddit_info::SubredditAbout;
    use crate::fetcher::model::user_info::UserAbout;
    use crate::fetcher::reddit::request::{SubredditAboutRequest, UserAboutRequest};
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

    use crate::fetcher::data_request::{DataSource, FetcherDataRequest, RedditFeedKind};
    use crate::fetcher::fetcher_error::FetcherError;
    use crate::fetcher::model::post_comments::PostComments;
    use crate::fetcher::model::posts::Posts;
    use crate::fetcher::model::user_posts::UserPosts;
    use crate::fetcher::reddit::request::feed_sorting::FeedSorting;
    use serial_test::serial;

    lazy_static! {
        static ref HTTP: Client = ClientBuilder::new().user_agent("RMoods").build().unwrap();
    }

    async fn get_fetcher() -> RMoodsFetcher {
        dotenvy::dotenv().ok();
        RMoodsFetcher::new(HTTP.clone()).await.unwrap()
    }

    #[tokio::test]
    #[serial]
    async fn fetch_about_user() {
        let mut fetcher = get_fetcher().await;

        let request = UserAboutRequest {
            username: "spez".to_string(),
        };
        let user = fetcher.fetch_about::<UserAbout>(request).await.unwrap();
        assert_eq!(user.info.name, "spez");
    }

    #[tokio::test]
    #[serial]
    async fn fetch_about_nonexistent_user() {
        let mut fetcher = get_fetcher().await;

        let request = UserAboutRequest {
            username: random_string(20),
        };
        let user = fetcher.fetch_about::<UserAbout>(request).await;
        assert!(user.is_err());
    }

    #[tokio::test]
    #[serial]
    async fn fetch_about_subreddit() {
        let mut fetcher = get_fetcher().await;

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
    #[serial]
    async fn fetch_about_nonexistent_subreddit() {
        let mut fetcher = get_fetcher().await;

        let request = SubredditAboutRequest {
            subreddit: random_string(20),
        };
        let sub = fetcher.fetch_about::<SubredditAbout>(request).await;
        assert!(sub.is_err());
    }

    #[tokio::test]
    #[serial]
    async fn fetch_feed_subreddit() {
        let mut fetcher = get_fetcher().await;

        let request = FetcherDataRequest {
            feed_kind: RedditFeedKind::SubredditPosts,
            data_sources: vec![DataSource {
                name: "programming".to_string(),
                post_id: None,
                share: 100,
            }],
            size: 5,
            sort_by: FeedSorting::New,
        };

        let (posts, requests_made) = fetcher.fetch_feed::<Posts>(request).await.unwrap();

        assert_eq!(requests_made, 5);
        assert!(posts.list.len() >= 1);
        assert!(posts.list.iter().all(|p| p.subreddit == "programming"));
    }

    #[tokio::test]
    #[serial]
    async fn fetch_feed_user() {
        let mut fetcher = get_fetcher().await;

        let request = FetcherDataRequest {
            feed_kind: RedditFeedKind::UserPosts,
            data_sources: vec![DataSource {
                name: "spez".to_string(),
                post_id: None,
                share: 100,
            }],
            size: 5,
            sort_by: FeedSorting::New,
        };

        let (user_posts, _) = fetcher.fetch_feed::<UserPosts>(request).await.unwrap();

        assert!(user_posts.posts.len() >= 1);
        assert!(user_posts.posts.iter().all(|p| p.author == "spez"));
    }

    #[tokio::test]
    #[serial]
    async fn fetch_feed_post() {
        let mut fetcher = get_fetcher().await;

        let request = FetcherDataRequest {
            feed_kind: RedditFeedKind::PostComments,
            data_sources: vec![DataSource {
                name: "announcements".to_string(),
                post_id: Some("7jsyqt".to_string()),
                share: 100,
            }],
            size: 5,
            sort_by: FeedSorting::New,
        };

        let (posts, _) = fetcher.fetch_feed::<PostComments>(request).await.unwrap();

        assert!(posts.list.len() >= 1);
        assert!(posts.list.iter().all(|p| p.subreddit == "announcements"));
    }

    #[tokio::test]
    #[serial]
    async fn fetch_feed_nonexistent_user() {
        let mut fetcher = get_fetcher().await;

        let request = FetcherDataRequest {
            feed_kind: RedditFeedKind::UserPosts,
            data_sources: vec![DataSource {
                name: random_string(20),
                post_id: None,
                share: 100,
            }],
            size: 5,
            sort_by: FeedSorting::New,
        };

        let posts = fetcher.fetch_feed::<Posts>(request).await.unwrap();

        assert!(posts.0.list.is_empty())
    }

    #[tokio::test]
    #[serial]
    async fn fetch_feed_nonexistent_subreddit() {
        let mut fetcher = get_fetcher().await;

        let request = FetcherDataRequest {
            feed_kind: RedditFeedKind::SubredditPosts,
            data_sources: vec![DataSource {
                name: random_string(20),
                post_id: None,
                share: 100,
            }],
            size: 5,
            sort_by: FeedSorting::New,
        };

        let posts = fetcher.fetch_feed::<Posts>(request).await.unwrap();

        assert!(posts.0.list.is_empty())
    }

    #[tokio::test]
    #[serial]
    async fn fetch_feed_nonexistent_post() {
        let mut fetcher = get_fetcher().await;

        let request = FetcherDataRequest {
            feed_kind: RedditFeedKind::PostComments,
            data_sources: vec![DataSource {
                name: "programming".to_string(),
                post_id: Some(random_string(20)),
                share: 100,
            }],
            size: 5,
            sort_by: FeedSorting::New,
        };

        let posts = fetcher.fetch_feed::<PostComments>(request).await;

        assert!(posts.is_err());
        assert!(matches!(
            posts.err().unwrap(),
            FetcherError::RedditApiError(_)
        ));
    }
}
