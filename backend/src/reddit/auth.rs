use log::debug;
use reqwest::Client;
use serde::Deserialize;
use std::time::SystemTime;

use super::error::RedditError;

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
    pub fn token(&self) -> &str {
        &self.token
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
    pub async fn fetch_access_token(
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