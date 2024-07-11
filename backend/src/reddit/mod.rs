use axum::http::HeaderValue;
use futures::executor::block_on;
use log::{debug, warn};
use project_root::get_project_root;
use reqwest::{Client, Request, Response};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fs::File, io::BufReader, mem::MaybeUninit, path::PathBuf, time::SystemTime};

pub mod model;

pub enum FeedSorting {
    Hot,
    New,
    Top,
    Rising,
    Controversial,
}

impl FeedSorting {
    pub const fn as_str(&self) -> &str {
        match self {
            FeedSorting::Hot => "hot",
            FeedSorting::New => "new",
            FeedSorting::Top => "top",
            FeedSorting::Rising => "rising",
            FeedSorting::Controversial => "controversial",
        }
    }
}

/// Get the current system time in seconds since UNIX EPOCH
fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/// Reddit API access token.
/// Generated from a RedditApp client_id and client_secret pair.
/// The token is valid for `expires_in` seconds.
/// The token is created at `created_at` seconds since UNIX EPOCH.
#[derive(Deserialize, Debug, Clone)]
pub struct RedditAccessToken {
    #[serde(rename = "access_token")]
    token: String,
    expires_in: u64,
    #[serde(default = "get_sys_time_in_secs")]
    created_at: u64,
}

impl RedditAccessToken {
    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        let now = get_sys_time_in_secs();
        debug!(
            "Token created at: {}, expires in: {}. Now it's {}",
            self.created_at, self.expires_in, now
        );
        self.created_at + self.expires_in < now // expired if expiration date was before now
    }
    /// Get the token string
    pub fn token(&self) -> String {
        self.token.clone()
    }
}

/// Reddit app data.
/// Contains a client_id and client_secret pair.
/// Used to authenticate with the Reddit API, we make many of them to avoid rate limiting.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct RedditApp {
    pub client_id: String,
    pub client_secret: String,
}

impl RedditApp {
    /// Fetch an access token from the Reddit API for that particular app
    pub async fn fetch_access_token(
        &self,
        http_client: &Client,
    ) -> anyhow::Result<RedditAccessToken> {
        let req = http_client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(self.client_id.as_str(), Some(self.client_secret.as_str())) // basic http auth
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body("grant_type=client_credentials") // for script apps
            .build()?;

        // Send the request and parse the response
        let res = http_client
            .execute(req)
            .await?
            .json::<RedditAccessToken>()
            .await?;
        Ok(res)
    }
}

/// Manages a collection of RedditApp clients and their access tokens.
///
/// Multiplexes requests to the Reddit API between the clients.
#[derive(Debug)]
pub struct RedditConnection {
    client: RedditApp,
    access_token: RedditAccessToken,
    http: Client,
}

impl RedditConnection {
    /// Reddit API URL
    const API_HOSTNAME: &'static str = "oauth.reddit.com";

    fn read_credentials() -> RedditApp {
        let root = get_project_root().expect("Should be able to determine project root");
        let path = PathBuf::from(root).join(".reddit-credentials.json");
        let file = File::open(path).expect("credentials should be present at backend root");
        let reader = BufReader::new(file);
        let json: Value =
            serde_json::from_reader(reader).expect("credentials should be valid JSON");

        let app_json = json
            .as_array()
            .expect("credentials contains an array")
            .get(0)
            .expect("At least one client")
            .clone();

        serde_json::from_value(app_json).expect("object is parsable into RedditApp")
    }

    /// Read credentials and create a collection of client authentication data
    /// from .reddit-credentials.json in backend root (src/backend/)
    pub async fn new() -> RedditConnection {
        let http = reqwest::ClientBuilder::new()
            .user_agent("RMoods")
            .build()
            .expect("Build HTTP Client");
        let client = Self::read_credentials();
        let access_token = client.fetch_access_token(&http).await.expect("Fetch initial token");
        println!("Before access token");

        RedditConnection {
            client,
            access_token,
            http,
        }
    }

    /// Execute a request to the Reddit API
    ///
    /// This is probably a temporary solution and I'll write specific methods for each API endpoint we need
    async fn execute(&mut self, mut req: Request) -> anyhow::Result<Response> {
        // If the current token is fine, use the current one. Otherwise fetch a new one and set it as the current
        if self.access_token.is_expired() {
            self.access_token = self.client.fetch_access_token(&self.http).await?;
        }

        // check if the request goes to the Reddit API
        assert!(req.url().domain().is_some_and(|d| d == Self::API_HOSTNAME));

        // Authorize the request
        let value = format!("bearer {}", self.access_token.token);
        req.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&value).expect("Valid token to header conversion"),
        );

        Ok(self.http.execute(req).await?)
    }

    /// TODO
    pub async fn fetch_subreddit(
        &mut self,
        subreddit: &str,
        feed_sorting: FeedSorting,
    ) -> anyhow::Result<Response> {
        let url = format!(
            "https://{}/r/{}/{}.json",
            Self::API_HOSTNAME,
            subreddit,
            feed_sorting.as_str()
        );
        let req = self.http.get(&url).build()?;
        self.execute(req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    #[test]
    fn test_read_credentials() {
        let _ = RedditConnection::read_credentials();
    }

    #[tokio::test]
    async fn test_app_can_fetch_access_token() {
        println!("Start");
        let conn = RedditConnection::new().await;
        println!("Conn created");
        let _ = conn.client.fetch_access_token(&conn.http).await;
    }

    #[tokio::test]
    async fn test_fetch_subreddit() {
        let mut conn = RedditConnection::new().await;
        let _ = conn
            .fetch_subreddit("all", FeedSorting::Hot)
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_execute_invalid_hostname() {
        let mut conn = RedditConnection::new().await;
        let req = conn.http
            .get("https://www.reddit.com/api/v1/me")
            .build()
            .unwrap();
        conn.execute(req).await.unwrap();
    }

    #[tokio::test]
    async fn test_execute_valid_hostname() {
        let mut conn = RedditConnection::new().await;
        let req = conn.http
            .get("https://oauth.reddit.com/api/v1/me")
            .build()
            .unwrap();
        conn.execute(req).await.unwrap();
    }
}
