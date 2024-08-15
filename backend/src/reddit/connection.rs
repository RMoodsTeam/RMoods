use std::time::SystemTime;

use log::{info, warn};
use log_derive::logfn;
use serde_json::Value;

use super::{
    auth::{RedditAccessToken, RedditApp},
    error::RedditError,
    request::RedditRequest,
};

/// Manages a collection of RedditApp clients and their access tokens.
///
/// In the future, it's planned to multiplex Reddit requests by using multiple apps and their request quotas all at once.
#[derive(Debug, Clone)]
pub struct RedditConnection {
    /// For now, it's a single app
    pub(super) client: RedditApp,
    /// Token from the app
    pub(super) access_token: RedditAccessToken,
    /// The connection's own HTTP client, decoupled from our main app. Can remove, but it would hurt performance a bit when making many requests.
    pub(super) http: reqwest::Client,
}

impl RedditConnection {
    /// Reddit API hostname
    pub const API_HOSTNAME: &'static str = "oauth.reddit.com";

    /// Read credentials from the environment and create a new [RedditConnection]
    #[logfn(err = "ERROR", fmt = "Failed to create RedditConnection: {:?}")]
    pub async fn new(http: reqwest::Client) -> Result<RedditConnection, RedditError> {
        let id = std::env::var("CLIENT_ID").expect("CLIENT_ID should be set");
        let secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET should be set");

        assert!(!id.is_empty());
        assert!(!secret.is_empty());

        let client = RedditApp::new(id, secret);

        info!("Fetching initial access token");
        let access_token = client.fetch_access_token(&http).await?;
        info!("Done fetching access token");

        Ok(RedditConnection {
            client,
            access_token,
            http,
        })
    }

    /// Execute a request to the Reddit API.
    ///
    /// Temporarily public for testing and debugging in the `api/debug.rs` module.
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
            .bearer_auth(&self.access_token.token())
            .build()?;

        let start = SystemTime::now();
        let res = self.http.execute(req).await?;
        let elapsed = SystemTime::now().duration_since(start).unwrap();

        let header_log = create_ratelimit_log(&res);
        info!("Headers: {}", header_log.trim_end());

        info!("Data fetched successfully. Took {:?}", elapsed);

        let json = res.json().await?;
        Ok(json)
    }
}

/// Read Reddit headers from the provided [reqwest::Request] and return a string ready to log.
/// Used for monitoring API usage.
///
/// The string contains information about:
/// * Number of remaining requests in the current period (1000s)
/// * Number of seconds until new period
/// * Number of requests used in the current period
fn create_ratelimit_log(res: &reqwest::Response) -> String {
    let ratelimit_headers = const {
        [
            "x-ratelimit-remaining",
            "x-ratelimit-reset",
            "x-ratelimit-used",
        ]
    };
    res.headers()
        .iter()
        .filter(|h| ratelimit_headers.contains(&h.0.as_str()))
        .fold(String::from("\n"), |acc, (k, v)| {
            let s = format!("{}: {}\n", k, v.to_str().unwrap());
            acc + s.as_str()
        })
}
