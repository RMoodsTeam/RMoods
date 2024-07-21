use log::{debug, info, warn};
use log_derive::logfn;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::time::SystemTime;

use self::reddit_error::RedditError;

pub mod model;
pub mod reddit_error;
mod tests;

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
}

/// Reddit app data.
/// Contains a client_id and client_secret pair.
/// Used to authenticate with the Reddit API.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct RedditApp {
    pub client_id: String,
    pub client_secret: String,
}

impl RedditApp {
    pub fn new(client_id: String, client_secret: String) -> Self {
        RedditApp {
            client_id,
            client_secret,
        }
    }
    
    /// Fetch an access token from the Reddit API for that particular app
    async fn fetch_access_token(
        &self,
        http_client: &Client,
    ) -> Result<RedditAccessToken, RedditError> {
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

/// Enum representing a request to the Reddit API
/// We can request information about a subreddit, posts from a subreddit, posts from a user, or information about a user
#[derive(Debug)]
pub enum RedditRequest {
    SubredditPosts(String),
    SubredditInfo(String),
    UserPosts(String),
    UserInfo(String),
    PostComments { subreddit: String, post_id: String },
}

/// Manages a collection of RedditApp clients and their access tokens.
///
/// Multiplexes requests to the Reddit API between the clients.
#[derive(Debug, Clone)]
pub struct RedditConnection {
    client: RedditApp,
    access_token: RedditAccessToken,
    http: Client,
}

impl RedditRequest {
    fn url(&self) -> String {
        let host = RedditConnection::API_HOSTNAME;
        type R = RedditRequest;
        match self {
            R::SubredditPosts(sub_name) => {
                format!("https://{}/r/{}/hot.json", host, sub_name)
            }
            R::SubredditInfo(sub_name) => {
                format!("https://{}/r/{}/about.json", host, sub_name)
            }
            R::UserPosts(username) => {
                format!("https://{}/user/{}.json", host, username)
            }
            R::UserInfo(username) => {
                format!("https://{}/user/{}/about.json", host, username)
            }
            R::PostComments { subreddit, post_id } => {
                format!("https://{}/r/{}/comments/{}.json", host, subreddit, post_id)
            }
        }
    }
}

impl RedditConnection {
    /// Reddit API URL
    const API_HOSTNAME: &'static str = "oauth.reddit.com";
    
    /// Read credentials and create a collection of client authentication data
    /// from .reddit-credentials.json in backend root (src/backend/)
    #[logfn(err = "ERROR", fmt = "Failed to create RedditConnection: {:?}")]
    pub async fn new(http: Client) -> Result<RedditConnection, RedditError> {
        let id = std::env::var("CLIENT_ID").expect("CLIENT_ID should be set");
        let secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET should be set");
        
        let client = RedditApp::new(id, secret);

        info!("Fetching initial access token");
        let access_token = client.fetch_access_token(&http).await?;
        info!("Done fetching access token");

        Ok(RedditConnection {
            client,
            access_token,
            http
        })
    }

    /// Execute a request to the Reddit API
    ///
    /// Temporarily public for testing
    #[logfn(err = "ERROR", fmt = "Failed to execute request: {:?}")]
    pub async fn fetch_raw(&mut self, request: RedditRequest) -> Result<Value, RedditError> {
        if self.access_token.is_expired() {
            warn!("Access token expired, fetching new one");
            self.access_token = self.client.fetch_access_token(&self.http).await?;
            info!("New access token fetched");
        }

        let url = request.url();
        info!("Fetching data from: {}", url);

        let req = self
            .http
            .get(url)
            .bearer_auth(&self.access_token.token)
            .build()?;

        let start = SystemTime::now();
        let res = self.http.execute(req).await?.json().await?;
        let elapsed = SystemTime::now().duration_since(start).unwrap();
        info!("Data fetched successfully. Took {:?}", elapsed);
        Ok(res)
    }
}
