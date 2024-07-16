use axum::{
    extract::{Query, State},
    Json,
};
use lipsum::lipsum;
use log::warn;
use log_derive::logfn;
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{
    app_error::AppError,
    reddit::{
        model::{
            listing::KindContainer,
            subreddit_info::{RedditSubredditInfo, RedditSubredditInfoRaw},
        },
        RedditRequest,
    },
    AppState,
};

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

    let info = serde_json::from_value::<RedditSubredditInfoRaw>(json).unwrap();

    Ok(Json(info.data))
}

type R = Vec<KindContainer>;

pub async fn subreddit_comments(
    State(mut state): State<AppState>,
    Query(params): Query<PlainParams>,
) -> Result<Json<R>, AppError> {
    let r = params
        .get("r")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `subreddit` parameter"))?
        .to_string();

    let p = params
        .get("p")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `post` parameter"))?
        .to_string();

    let req = RedditRequest::PostComments {
        subreddit: r,
        post_id: p,
    };

    let json = state.reddit.fetch_raw(req).await?;
    warn!("Fetched");

    let val = serde_json::from_value::<R>(json).unwrap();
    Ok(Json(val))

    // let post_json = json.as_array().unwrap().get(0).unwrap();

    // let parsed = serde_json::from_value::<P>(post_json.clone()).expect("Parse post");

    // let comments_json = json.as_array().unwrap().get(1).unwrap();

    // // Write the comments to a file
    // let s = serde_json::to_string_pretty(comments_json).unwrap();
    // let file = std::fs::File::create("comments.json").unwrap();
    // let mut writer = std::io::BufWriter::new(file);
    // writer.write_all(s.as_bytes()).unwrap();
    // writer.flush().unwrap();
    // warn!("Wrote comments to file");

    // let comments_container = serde_json::from_value(comments_json.clone()).unwrap();
}
