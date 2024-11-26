use crate::reddit_fetcher::reddit::connection::InnerFetchError;
use jsonwebtoken::get_current_timestamp;

pub const INITIAL_REQUESTS: u64 = 1000;

#[derive(Debug)]
pub struct RatelimitHeaders {
    remaining: u64,
    reset: u64,
    used: u64,
    created_at: u64,
}

impl RatelimitHeaders {
    pub fn new() -> Self {
        RatelimitHeaders {
            remaining: INITIAL_REQUESTS,
            reset: 0,
            used: 0,
            created_at: get_current_timestamp(),
        }
    }
}

/// Extract Reddit ratelimit headers from the provided [reqwest::Request]
pub fn get_ratelimit_headers(res: &reqwest::Response) -> Result<RatelimitHeaders, InnerFetchError> {
    let get_header_or_err = |header: &str| -> Result<u64, InnerFetchError> {
        res.headers()
            .get(header)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .ok_or_else(|| InnerFetchError::MissingRateLimitHeader(header.to_string()))
    };

    Ok(RatelimitHeaders {
        remaining: get_header_or_err("x-ratelimit-remaining")?,
        reset: get_header_or_err("x-ratelimit-reset")?,
        used: get_header_or_err("x-ratelimit-used")?,
        created_at: get_current_timestamp(),
    })
}
