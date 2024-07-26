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
/// Multiplexes requests to the Reddit API between the clients.
#[derive(Debug, Clone)]
pub struct RedditConnection {
    pub(super) client: RedditApp,
    pub(super) access_token: RedditAccessToken,
    pub(super) http: reqwest::Client,
}

impl RedditConnection {
    /// Reddit API URL
    pub const API_HOSTNAME: &'static str = "oauth.reddit.com";

    /// Read credentials and create a collection of client authentication data
    /// from .reddit-credentials.json in backend root (src/backend/)
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
