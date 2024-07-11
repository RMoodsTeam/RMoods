use axum::http::HeaderValue;
use log::{debug, warn};
use project_root::get_project_root;
use reqwest::{Request, Response};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf, time::SystemTime};

use crate::REQWEST_CLIENT;

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
#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct RedditApp {
    pub client_id: String,
    pub client_secret: String,
}

impl RedditApp {
    /// Fetch an access token from the Reddit API for that particular app
    pub async fn fetch_access_token(&self) -> anyhow::Result<RedditAccessToken> {
        let req = REQWEST_CLIENT
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(self.client_id.as_str(), Some(self.client_secret.as_str())) // basic http auth
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body("grant_type=client_credentials") // for script apps
            .build()?;

        // Send the request and parse the response
        let res = REQWEST_CLIENT
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
    pub clients: HashMap<RedditApp, Option<RedditAccessToken>>,
    access_token: Option<RedditAccessToken>,
}

impl RedditConnection {
    /// Reddit API URL
    const API_HOSTNAME: &'static str = "oauth.reddit.com";

    fn read_credentials() -> HashMap<RedditApp, Option<RedditAccessToken>> {
        let root = get_project_root().expect("Should be able to determine project root");
        let path = PathBuf::from(root).join(".reddit-credentials.json");
        let file = File::open(path).expect("credentials should be present at backend root");
        let reader = BufReader::new(file);
        let json: Value =
            serde_json::from_reader(reader).expect("credentials should be valid JSON");

        println!("{:?}", serde_json::to_string_pretty(&json).unwrap());

        // Parse the JSON into a collection of RedditApp clients
        json.as_array()
            .expect("credentials file should only contain an array")
            .iter()
            .map(|i: &Value| {
                (
                    serde_json::from_value::<RedditApp>(i.clone()).expect("Parsable client data"),
                    None,
                )
            })
            .collect()
    }

    /// Read credentials and create a collection of client authentication data
    /// from .reddit-credentials.json in backend root (src/backend/)
    pub fn new() -> RedditConnection {
        let clients = Self::read_credentials();
        assert!(!clients.is_empty());

        RedditConnection {
            clients,
            access_token: None,
        }
    }

    /// Determine which token (from which app) will be used for the next request
    fn pick_app(&self) -> &RedditApp {
        self.clients
            .keys()
            .next()
            .expect("There is at least one client")
    }

    /// Execute a request to the Reddit API
    ///
    /// This is probably a temporary solution and I'll write specific methods for each API endpoint we need
    async fn execute(&mut self, mut req: Request) -> anyhow::Result<Response> {
        // If the current token is fine, use the current one. Otherwise fetch a new one and set it as the current
        let token = match &self.access_token {
            Some(t) if !t.is_expired() => t.clone(),
            _ => {
                warn!("Token invalid, refetching...");
                let t = self.pick_app().fetch_access_token().await?;
                self.access_token = Some(t.clone());
                t
            }
        };
        if token.is_expired() {
            todo!("Handle expired tokens");
        }

        // check if the request goes to the Reddit API
        assert!(req.url().domain().is_some_and(|d| d == Self::API_HOSTNAME));

        // Authorize the request
        let value = format!("bearer {}", token.token);
        req.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&value).expect("Valid token to header conversion"),
        );

        Ok(REQWEST_CLIENT.execute(req).await?)
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
        let req = REQWEST_CLIENT.get(&url).build()?;
        self.execute(req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref CONN: Mutex<RedditConnection> = Mutex::new(RedditConnection::new());
    }

    #[test]
    fn test_read_credentials() {
        let creds = RedditConnection::read_credentials();
        println!("{:?}", creds);
    }

    #[tokio::test]
    async fn test_all_apps_can_fetch_access_tokens() {
        let conn = CONN.lock().unwrap();
        for client in conn.clients.keys() {
            let _ = client.fetch_access_token().await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_fetch_subreddit() {
        let mut conn = CONN.lock().unwrap();
        let res = conn
            .fetch_subreddit("all", FeedSorting::Hot)
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_execute_invalid_hostname() {
        let mut conn = RedditConnection::new();
        let req = REQWEST_CLIENT
            .get("https://www.reddit.com/api/v1/me")
            .build()
            .unwrap();
        conn.execute(req).await.unwrap();
    }

    #[tokio::test]
    async fn test_execute_valid_hostname() {
        let mut conn = RedditConnection::new();
        let req = REQWEST_CLIENT
            .get("https://oauth.reddit.com/api/v1/me")
            .build()
            .unwrap();
        conn.execute(req).await.unwrap();
    }
}
