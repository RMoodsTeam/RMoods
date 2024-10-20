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
    reddit::request::params::FeedSorting,
    rmoods_fetcher::{
        rmoods_request::{
            DataSource, RMoodsNlpRequest, RMoodsReportType, RedditFeedKind, RequestSize,
        },
        PostComments, Posts, UserPosts,
    },
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
) -> Result<Json<PostComments>, AppError> {
    let request = RMoodsNlpRequest {
        resource_kind: RedditFeedKind::PostComments,
        report_types: vec![RMoodsReportType::Sarcasm],
        data_sources: vec![DataSource {
            name: "interesting".to_string(),
            post_id: Some("1g7e1g6".to_string()),
            share: 1.0,
        }],
        size: RequestSize::Custom(10),
        sorting: FeedSorting::New,
    };

    let (mut data, requests_left) = state
        .fetcher
        .fetch_feed::<PostComments>(request)
        .await
        .unwrap();

    debug!("Returning {} post comments", data.list.len());

    let more_comments = state
        .fetcher
        .fetch_more_comments(&data.more, requests_left)
        .await
        .unwrap();

    data.list.extend(more_comments);

    Ok(Json(data))
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
pub async fn subreddit_posts(State(mut state): State<AppState>) -> Result<Json<Posts>, AppError> {
    let request = RMoodsNlpRequest {
        resource_kind: RedditFeedKind::PostComments,
        report_types: vec![RMoodsReportType::Sarcasm],
        data_sources: vec![DataSource {
            name: "nosleep".to_string(),
            post_id: None,
            share: 1.0,
        }],
        size: RequestSize::Custom(30),
        sorting: FeedSorting::New,
    };

    let (data, _) = state.fetcher.fetch_feed::<Posts>(request).await.unwrap();

    debug!("Returning {} subreddit posts", data.list.len());

    Ok(Json(data))
}

#[utoipa::path(get, path = "/api/debug/user_posts", responses(), params())]
pub async fn user_posts(State(mut state): State<AppState>) -> Result<Json<UserPosts>, AppError> {
    let request = RMoodsNlpRequest {
        resource_kind: RedditFeedKind::UserPosts,
        report_types: vec![RMoodsReportType::Sarcasm],
        data_sources: vec![DataSource {
            name: "spez".to_string(),
            post_id: None,
            share: 1.0,
        }],
        size: RequestSize::Custom(10),
        sorting: Default::default(),
    };

    let (data, _) = state
        .fetcher
        .fetch_feed::<UserPosts>(request)
        .await
        .unwrap();

    debug!("Returning {} user posts", data.posts.len());
    debug!("Returning {} user comments", data.comments.len());

    Ok(Json(data))
}
