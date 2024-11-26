use super::{
    auth::{RedditAccessToken, RedditApp},
    error::RedditError,
    model::{MoreComments, RawComment, RawContainer},
    ratelimit_headers,
    request::RedditRequest,
};
use crate::reddit_fetcher::reddit::ratelimit_headers::RatelimitHeaders;
use http::StatusCode;
use log_derive::logfn;
use serde_json::Value;
use std::sync::Arc;
use std::time::SystemTime;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug, Error)]
pub enum InnerFetchError {
    #[error("HTTP Error: {0:?}")]
    HttpError(#[from] reqwest::Error),

    #[error("Reddit Error: {0:?}")]
    RedditError(#[from] RedditError),

    #[error("Missing rate limit header: {0}")]
    MissingRateLimitHeader(String),
}

/// Manages a collection of RedditApp clients and their access tokens.
///
/// In the future, it's planned to multiplex Reddit requests by using multiple apps and their request quotas all at once.
#[derive(Debug, Clone)]
pub struct RedditConnection {
    /// For now, it's a single app
    pub(crate) client: RedditApp,
    /// Token from the app
    pub(crate) access_token: RedditAccessToken,
    /// The connection's own HTTP client, decoupled from our main app. Can remove, but it would hurt performance a bit when making many requests.
    pub(crate) http: reqwest::Client,
    /// Rate limit headers as we last got them from Reddit, along with a timestamp of when we got them
    pub(crate) ratelimit_headers: Arc<RwLock<RatelimitHeaders>>,
}

impl RedditConnection {
    /// Read credentials from the environment and create a new [RedditConnection]
    #[logfn(
        err = "ERROR",
        fmt = "Fetcher - Failed to create RedditConnection: {:?}"
    )]
    pub async fn new(http: reqwest::Client) -> Result<RedditConnection, RedditError> {
        let id = std::env::var("CLIENT_ID").expect("CLIENT_ID should be set");
        let secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET should be set");

        if id.is_empty() || secret.is_empty() {
            return Err(RedditError::FailedToFetchAccessToken(
                "CLIENT_ID or CLIENT_SECRET is empty".to_string(),
            ));
        }

        let client = RedditApp::new(id, secret);

        log::info!("Fetching initial access token");
        let access_token = client
            .fetch_access_token(&http)
            .await
            .or_else(|e| Err(RedditError::FailedToFetchAccessToken(e.to_string())))?;
        log::debug!("Access token: {:?}", access_token);
        log::info!("Done fetching access token");

        Ok(RedditConnection {
            client,
            access_token,
            http,
            ratelimit_headers: Arc::new(RwLock::new(RatelimitHeaders::new())),
        })
    }

    #[logfn(err = "ERROR", fmt = "Failed to refresh access token: {0:?}")]
    async fn refresh_access_token(&mut self) -> Result<(), RedditError> {
        if self.access_token.is_expired() {
            log::warn!("Access token expired, fetching new one");
            self.access_token = self.client.fetch_access_token(&self.http).await?;
            log::info!("New access token fetched");
        }
        Ok(())
    }

    pub fn redirect_policy() -> reqwest::redirect::Policy {
        reqwest::redirect::Policy::custom(|attempt| {
            if attempt.url().path().contains("subreddits/search.json") {
                attempt.error(
                    "Redirecting to subreddits/search.json because subreddit does not exist.",
                )
            } else {
                attempt.follow()
            }
        })
    }

    #[logfn(err = "ERROR", fmt = "Failed inner_fetch: {0:?}")]
    async fn inner_fetch(
        &mut self,
        url: &str,
        query: Vec<(&str, String)>,
    ) -> Result<Value, InnerFetchError> {
        log::info!("Fetching data from: {url:?}. Query params: {query:?}");

        let req = self
            .http
            .get(url)
            .query(&query)
            .bearer_auth(&self.access_token.token())
            .build()?;

        let start = SystemTime::now();
        let res = self.http.execute(req).await;
        let elapsed = SystemTime::now().duration_since(start).unwrap();

        let res = if let Err(e) = &res {
            // we get a redirection error only if the subreddit does not exist, see [RedditConnection::redirect_policy]
            if e.is_redirect() {
                log::warn!(
                    "Redirected to subreddits/search.json because subreddit does not exist."
                );
                return Err(InnerFetchError::RedditError(RedditError::ResourceNotFound(
                    url.to_string(),
                )));
            }
            res?
        } else {
            res?
        };

        let ratelimit_headers = ratelimit_headers::get_ratelimit_headers(&res)?;
        log::info!("Rate Limits: {:?}", ratelimit_headers);
        {
            *self.ratelimit_headers.write().await = ratelimit_headers;
        }

        log::info!("Data fetched successfully. Took {:?}", elapsed);

        if !res.status().is_success() {
            log::warn!("Failed to fetch data: {:?}", res);
            match res.status() {
                StatusCode::NOT_FOUND => {
                    return Err(InnerFetchError::RedditError(RedditError::ResourceNotFound(
                        url.to_string(),
                    )));
                }
                _ => {
                    log::warn!("Received error status code: {:?}", res);
                }
            }
        }

        Ok(res.json::<Value>().await?)
    }

    /// Execute a request to the Reddit API.
    ///
    /// Temporarily public for testing and debugging in the `api/debug.rs` module.
    #[logfn(err = "ERROR", fmt = "Failed to execute request: {:?}")]
    pub async fn fetch_raw(
        &mut self,
        request: impl RedditRequest,
    ) -> Result<(RawContainer, Option<String>), RedditError> {
        self.refresh_access_token().await?;
        let (url, query) = request.to_request_parts();

        let json = self.inner_fetch(&url, query).await?;

        // Special case for comments, as they are wrapped in an array
        // First element of said array is the post, second is the comments
        // We only care about the comments.
        // [Post, Listing<Comment>]
        if json.is_array() {
            log::debug!("The returned JSON is an array, extracting comments");
            let comments_container = json.as_array().and_then(|a| a.get(1).cloned()).unwrap();
            let after = comments_container
                .get("after")
                .and_then(|a| a.as_str())
                .map(|s| s.to_string());
            let parsed = serde_json::from_value(comments_container);
            match parsed {
                Ok(parsed) => Ok((parsed, after)),
                Err(err) => {
                    log::warn!("Failed to parse comments: {:?}", err);
                    Err(RedditError::OtherRedditError(
                        "Failed to parse comments".to_string(),
                    ))
                }
            }
        } else {
            log::debug!("The returned JSON is not an array, parsing normally");
            let after = json
                .get("data")
                .and_then(|d| d.get("after"))
                .and_then(|a| a.as_str())
                .map(|s| s.to_string());

            let parsed = serde_json::from_value::<RawContainer>(json.clone())?;
            Ok((parsed, after))
        }
    }

    #[logfn(err = "ERROR", fmt = "Failed to fetch more comments: {0:?}")]
    pub async fn fetch_more_comments(
        &mut self,
        more: &MoreComments,
        requests_left: u16,
    ) -> Result<(Vec<RawComment>, u16), RedditError> {
        self.refresh_access_token().await?;
        let request_parts_vec = more.clone().into_request_parts();

        let mut comments = vec![];
        let mut requests_made = 0;

        for (url, query) in request_parts_vec {
            let json = self.inner_fetch(&url, query).await?;

            requests_made += 1;
            log::debug!("Comment requests made: {}/{}", requests_made, requests_left);

            let json_list = json
                .get("json")
                .and_then(|j| j.get("data"))
                .and_then(|d| d.get("things"))
                .cloned();

            // If the response is correct, we can extract the comments and add them to the list
            if let Some(list) = json_list {
                let list = serde_json::from_value::<Vec<RawContainer>>(list)?;
                let new_comments = list
                    .iter()
                    .filter_map(|c| match c {
                        RawContainer::Comment(c) => Some(*c.clone()),
                        _ => None,
                    })
                    .collect::<Vec<RawComment>>();
                comments.extend(new_comments);
            } else {
                return Err(RedditError::OtherRedditError(
                    "Expected json.data.things to be present in response to MoreComments request"
                        .to_string(),
                )
                .into());
            }
            if requests_made >= requests_left {
                log::debug!("No more comments - no requests left");
                break;
            }
        }

        Ok((comments, requests_made))
    }
}
