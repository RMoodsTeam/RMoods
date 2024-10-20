use axum::{
    extract::{Query, State},
    Json,
};
use lipsum::lipsum;
use log::debug;
use log_derive::logfn;
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{
    app_error::AppError,
    reddit::request::{PostCommentsRequest, SubredditPostsRequest, UserPostsRequest},
    rmoods_fetcher::{PostComments, Posts, RedditData, UserPosts},
    AppState,
};

use super::AnyParams;

/// Returns after a specified delay.
#[utoipa::path(
    get,
    path = "/api/debug/timeout",
    responses(
        (status = 200, description = "Timed out successfully"),
        (status = 400, description = "No parameter provided")
    ),
    params(
        ("t" = u64, description = "Time to wait in seconds", nullable = false)
    )
)]
#[logfn(err = "ERROR", fmt = "'timeout' failed: {:?}")]
pub async fn timeout(Query(params): Query<AnyParams>) -> Result<Json<Value>, AppError> {
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
    path = "/api/debug/lorem",
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
pub async fn lorem(Query(params): Query<AnyParams>) -> Result<Json<Value>, AppError> {
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

// #[utoipa::path(get, path = "/api/debug/subreddit_info", responses(), params())]
// pub async fn subreddit_info(
//     State(mut state): State<AppState>,
//     Query(params): Query<AnyParams>,
// ) -> Result<Json<SubredditInfo>, AppError> {
//     // let subreddit = params
//     //     .get("r")
//     //     .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `subreddit` parameter"))?;
//     // let req = SubredditInfoRequest {
//     //     subreddit: subreddit.to_string(),
//     // };
//     // let json = state.reddit.fetch_raw(req).await?;

//     // let parsed = SubredditInfo::from_reddit_container(json).unwrap();

//     // Ok(Json(parsed))
//     todo!();
// }

#[utoipa::path(get, path = "/api/debug/post_comments", responses(), params())]
pub async fn post_comments(
    State(mut state): State<AppState>,
    Query(params): Query<AnyParams>,
) -> Result<Json<PostComments>, AppError> {
    let r = params
        .get("r")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `r` parameter"))?
        .to_string();

    let id = params
        .get("id")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `id` parameter"))?
        .to_string();

    let req = PostCommentsRequest {
        subreddit: r,
        post_id: id,
        sorting: Default::default(),
        after: None,
    };

    let (data, after) = state.reddit.fetch_raw(req).await?;
    let parsed = PostComments::from_reddit_container(data).expect("second");
    dbg!(&parsed.more);

    Ok(Json(parsed))
}

// #[utoipa::path(get, path = "/api/debug/user_info", responses(), params())]
// pub async fn user_info(
//     State(mut state): State<AppState>,
//     Query(params): Query<AnyParams>,
// ) -> Result<Json<UserInfo>, AppError> {
//     // let user = params
//     //     .get("u")
//     //     .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `u` parameter"))?;
//     // let req = UserInfoRequest {
//     //     username: user.to_string(),
//     // };

//     // let json = state.reddit.fetch_raw(req).await?;

//     // let parsed = UserInfo::from_reddit_container(json).unwrap();
//     // Ok(Json(parsed))
//     todo!();
// }

#[utoipa::path(get, path = "/api/debug/subreddit_posts", responses(), params())]
pub async fn subreddit_posts(
    State(mut state): State<AppState>,
    Query(params): Query<AnyParams>,
) -> Result<Json<Posts>, AppError> {
    let subreddit = params
        .get("r")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `r` parameter"))?;
    let req = SubredditPostsRequest {
        subreddit: subreddit.into(),
        sorting: Default::default(),
        after: None,
    };
    let (data, after) = state.reddit.fetch_raw(req).await?;

    let parsed = Posts::from_reddit_container(data).unwrap();
    debug!("Returning {} subreddit posts", parsed.list.len());

    Ok(Json(parsed))
}

#[utoipa::path(get, path = "/api/debug/user_posts", responses(), params())]
pub async fn user_posts(
    State(mut state): State<AppState>,
    Query(params): Query<AnyParams>,
) -> Result<Json<UserPosts>, AppError> {
    let user = params
        .get("u")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `u` parameter"))?;
    let req = UserPostsRequest {
        username: user.into(),
        sorting: Default::default(),
        after: None,
    };

    let (data, after) = state.reddit.fetch_raw(req).await?;

    let parsed = UserPosts::from_reddit_container(data).unwrap();

    debug!("Returning {} user posts", parsed.posts.len());
    debug!("Returning {} user comments", parsed.comments.len());

    Ok(Json(parsed))
}
