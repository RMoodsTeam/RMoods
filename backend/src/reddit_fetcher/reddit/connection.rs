use http::StatusCode;
use log::{debug, info, warn};
use log_derive::logfn;
use serde_json::Value;
use std::time::SystemTime;

use super::{
    auth::{RedditAccessToken, RedditApp},
    error::RedditError,
    model::{MoreComments, RawComment, RawContainer},
    request::RedditResource,
};

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

        assert!(!id.is_empty());
        assert!(!secret.is_empty());

        let client = RedditApp::new(id, secret);

        info!("Fetching initial access token");
        let access_token = client.fetch_access_token(&http).await.or(Err(
            RedditError::FailedToFetchAccessToken(client.client_id.to_string()),
        ))?;
        debug!("Access token: {:?}", access_token);
        info!("Done fetching access token");

        Ok(RedditConnection {
            client,
            access_token,
            http,
        })
    }

    #[logfn(err = "ERROR", fmt = "Failed to refresh access token: {0}")]
    async fn refresh_access_token(&mut self) -> Result<(), RedditError> {
        if self.access_token.is_expired() {
            warn!("Access token expired, fetching new one");
            self.access_token = self.client.fetch_access_token(&self.http).await?;
            info!("New access token fetched");
        }
        Ok(())
    }

    #[logfn(err = "ERROR", fmt = "Failed inner_fetch: {0}")]
    async fn inner_fetch(
        &mut self,
        url: String,
        query: Vec<(&str, String)>,
    ) -> Result<Value, reqwest::Error> {
        info!("Fetching data from: {url:?}\nWith query params: {query:?}");

        let req = self
            .http
            .get(url)
            .query(&query)
            .bearer_auth(&self.access_token.token())
            .build()?;

        let start = SystemTime::now();
        let res = self.http.execute(req).await?;
        let elapsed = SystemTime::now().duration_since(start).unwrap();

        let header_log = create_ratelimit_log(&res);
        info!("Headers: {}", header_log.trim_end());

        info!("Data fetched successfully. Took {:?}", elapsed);

        Ok(res.json().await?)
    }

    /// Execute a request to the Reddit API.
    ///
    /// Temporarily public for testing and debugging in the `api/debug.rs` module.
    #[logfn(err = "ERROR", fmt = "Failed to execute request: {:?}")]
    pub async fn fetch_raw(
        &mut self,
        request: impl RedditResource,
    ) -> Result<(RawContainer, Option<String>), RedditError> {
        self.refresh_access_token().await?;
        let (url, query) = request.to_request_parts();

        let json = self.inner_fetch(url, query).await;

        let json = json.map_err(|err| match err.status() {
            Some(status) => match status {
                StatusCode::NOT_FOUND => RedditError::ResourceNotFound(request.resource_name()),
                _ => RedditError::HttpError(err),
            },
            None => RedditError::HttpError(err),
        })?;

        // Special case for comments, as they are wrapped in an array
        // First element of said array is the post, second is the comments
        // We only care about the comments.
        // [Post, Listing<Comment>]
        if json.is_array() {
            let comments_container = json.as_array().and_then(|a| a.get(1).cloned()).unwrap();
            let after = comments_container
                .get("after")
                .and_then(|a| a.as_str())
                .map(|s| s.to_string());
            Ok((serde_json::from_value(comments_container)?, after))
        } else {
            let after = json
                .get("data")
                .and_then(|d| d.get("after"))
                .and_then(|a| a.as_str())
                .map(|s| s.to_string());
            Ok((serde_json::from_value(json)?, after))
        }
    }

    #[logfn(err = "ERROR", fmt = "Failed to fetch more comments: {0}")]
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
            let json = self.inner_fetch(url, query).await;

            let json = json.map_err(|err| match err.status() {
                Some(status) => match status {
                    StatusCode::NOT_FOUND => {
                        RedditError::ResourceNotFound(format!("{}/children", more.parent_id))
                    }
                    _ => RedditError::HttpError(err),
                },
                None => RedditError::HttpError(err),
            })?;

            requests_made += 1;
            debug!("Comment requests made: {}/{}", requests_made, requests_left);

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
                return Err(RedditError::OtherJsonError(
                    "Expected json.data.things to be present in response to MoreComments request"
                        .to_string(),
                )
                .into());
            }
            if requests_made >= requests_left {
                debug!("No more comments - no requests left");
                break;
            }
        }

        Ok((comments, requests_made))
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
        .filter_map(|(k, v)| {
            if ratelimit_headers.contains(&k.as_str()) {
                Some(format!("{}: {}\n", k, v.to_str().unwrap()))
            } else {
                None
            }
        })
        .collect::<String>()
}
