use super::AnyParams;
use crate::reddit_fetcher::feed_request::{
    DataSource, FetcherFeedRequest, RMoodsReportType, RedditFeedKind, RequestSize,
};
use crate::reddit_fetcher::model::post_comments::PostComments;
use crate::reddit_fetcher::model::posts::Posts;
use crate::reddit_fetcher::model::subreddit_info::SubredditAbout;
use crate::reddit_fetcher::model::user_info::UserAbout;
use crate::reddit_fetcher::model::user_posts::UserPosts;
use crate::reddit_fetcher::reddit::request::params::FeedSorting;
use crate::reddit_fetcher::reddit::request::{SubredditAboutRequest, UserAboutRequest};
use crate::websocket::SystemMessage;
use crate::{app_error::AppError, AppState};
use axum::{
    extract::{Query, State},
    Json,
};
use lipsum::lipsum;
use log::{debug, info};
use log_derive::logfn;
use reqwest::StatusCode;
use serde_json::{json, Value};

#[utoipa::path(get, path = "/api/debug/subreddit_about", responses(), params())]
pub async fn subreddit_about(
    State(mut state): State<AppState>,
    Query(params): Query<AnyParams>,
) -> Result<StatusCode, AppError> {
    let subreddit = params
        .get("r")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `subreddit` parameter"))?;
    let req = SubredditAboutRequest {
        subreddit: subreddit.to_string(),
    };
    let about = state
        .fetcher
        .fetch_about::<SubredditAbout>(req)
        .await
        .unwrap();

    Ok(StatusCode::OK)
}

#[utoipa::path(get, path = "/api/debug/post_comments", responses(), params())]
pub async fn post_comments(State(mut state): State<AppState>) -> Result<StatusCode, AppError> {
    let request = FetcherFeedRequest {
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
    let requests_to_make = u16::from(request.size.clone());

    tokio::spawn(async move {
        let (mut data, requests_made) = state
            .fetcher
            .fetch_feed::<PostComments>(request)
            .await
            .unwrap();

        debug!("Returning {} post comments", data.list.len());

        let more_comments = state
            .fetcher
            .fetch_more_comments(&data.more, requests_to_make - requests_made)
            .await
            .unwrap();

        data.list.extend(more_comments);
        // Remove more comments, as they are already fetched and useless to consumers
        data.more.clear();

        info!("Returning {} post comments", data.list.len());

        state
            .system_tx
            .send(SystemMessage::ReportDone(Box::new(data)))
            .await
            .unwrap();
    });

    Ok(StatusCode::OK)
}

#[utoipa::path(get, path = "/api/debug/user_info", responses(), params())]
pub async fn user_about(
    State(mut state): State<AppState>,
    Query(params): Query<AnyParams>,
) -> Result<Json<UserAbout>, AppError> {
    let user = params
        .get("u")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `u` parameter"))?;
    let req = UserAboutRequest {
        username: user.to_string(),
    };
    let about = state.fetcher.fetch_about::<UserAbout>(req).await.unwrap();
    Ok(Json(about))
}

// TODO: Add proper response type for acknowledged requests
#[utoipa::path(get, path = "/api/debug/subreddit_posts", responses(), params())]
pub async fn subreddit_posts(State(mut state): State<AppState>) -> Result<StatusCode, AppError> {
    let request = FetcherFeedRequest {
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

    tokio::spawn(async move {
        log::info!("Spawning a new task to fetch subreddit posts");
        let (data, _) = state.fetcher.fetch_feed::<Posts>(request).await.unwrap();
        log::debug!("Returning {} subreddit posts", data.list.len());
        state
            .system_tx
            .send(SystemMessage::ReportDone(Box::new(data)))
            .await
            .unwrap();
        log::debug!("Sent the subreddit posts to the WebSocket service");
    });

    Ok(StatusCode::OK)
}

#[utoipa::path(get, path = "/api/debug/user_posts", responses(), params())]
pub async fn user_posts(State(mut state): State<AppState>) -> Result<StatusCode, AppError> {
    let request = FetcherFeedRequest {
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

    // TODO: If there are no requests, return appropriate message to the user

    tokio::spawn(async move {
        log::info!("Spawning a new task to fetch user posts");
        let (data, _) = state
            .fetcher
            .fetch_feed::<UserPosts>(request)
            .await
            .unwrap();
        debug!("Returning {} user posts", data.posts.len());
        debug!("Returning {} user comments", data.comments.len());
        state
            .system_tx
            .send(SystemMessage::ReportDone(Box::new(data)))
            .await
            .unwrap();
        log::debug!("Sent the subreddit posts to the WebSocket service");
    });

    Ok(StatusCode::OK)
}
