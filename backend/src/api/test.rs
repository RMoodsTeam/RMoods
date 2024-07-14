use axum::{
    extract::{Query, State},
    Json,
};
use lipsum::lipsum;
use log_derive::logfn;
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{app_error::AppError, reddit::{model::subreddit::{RedditSubredditInfo, RedditSubredditInfoWrapped}, RedditRequest}, AppState};

use super::PlainParams;

/// Returns after a specified delay.
#[utoipa::path(
    get,
    path = "/api/test/timeout",
    responses(
        (status = 200, description = "Timed out successfully"),
        (status = 400, description = "No parameter provided")
    ),
    params(
        ("t" = u64, description = "Time to wait in seconds", nullable = false)
    )
)]
#[logfn(err = "ERROR", fmt = "'timeout' failed: {:?}")]
pub async fn timeout(Query(params): Query<PlainParams>) -> Result<Json<Value>, AppError> {
    let t = params
        .get("t")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `t` parameter"))?
        .parse::<u64>()
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(t)).await;
    Ok(json!({
        "t" : t
    })
    .into())
}

/// Generates a random lorem ipsum text with a specified number of words.
/// Optionally waits for a specified time before responding.
#[utoipa::path(
    get,
    path = "/api/test/lorem",
    responses(
        (status = 200, description = "Timed out successfully, returned lorem ipsum text"),
        (status = 400, description = "Invalid request")
    ),
    params(
        ("words" = usize, description = "Number of words to generate", nullable = true),
        ("t" = u64, description = "Time to wait in seconds", nullable = true)
    )
)]
#[logfn(err = "ERROR", fmt = "'lorem' failed: {:?}")]
pub async fn lorem(Query(params): Query<PlainParams>) -> Result<Json<Value>, AppError> {
    let words = params
        .get("words")
        .unwrap_or(&"50".to_string())
        .parse::<usize>()
        .unwrap();
    let t = params
        .get("t")
        .unwrap_or(&"0".to_string())
        .parse::<u64>()
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(t)).await;

    Ok(json!({
        "message": lipsum(words),
        "t": t
    })
    .into())
}

pub async fn subreddit_info(
    State(mut state): State<AppState>,
    Query(params): Query<PlainParams>,
) -> Result<Json<RedditSubredditInfo>, AppError> {
    let subreddit = params
        .get("r")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `subreddit` parameter"))?;
    let req = RedditRequest::SubredditInfo(subreddit.into());
    let json = state.reddit.fetch_raw(req).await?;

    let info = serde_json::from_value::<RedditSubredditInfoWrapped>(json).unwrap();
    
    Ok(Json(info.data))
}
