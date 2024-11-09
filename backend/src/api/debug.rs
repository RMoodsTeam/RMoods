use super::AnyParams;
use crate::api::auth::google::GoogleUserInfo;
use crate::reddit_fetcher::feed_request::{
    DataSource, FetcherFeedRequest, RMoodsReportType, RedditFeedKind, RequestSize,
};
use crate::reddit_fetcher::fetcher_error::FetcherError;
use crate::reddit_fetcher::model::post_comments::PostComments;
use crate::reddit_fetcher::model::posts::Posts;
use crate::reddit_fetcher::model::subreddit_info::SubredditAbout;
use crate::reddit_fetcher::model::user_info::UserAbout;
use crate::reddit_fetcher::model::user_posts::UserPosts;
use crate::reddit_fetcher::reddit::request::params::FeedSorting;
use crate::reddit_fetcher::reddit::request::{SubredditAboutRequest, UserAboutRequest};
use crate::rmoods::report_ack::ReportAck;
use crate::websocket::SystemMessage;
use crate::websocket::SystemMessage::ReportError;
use crate::{app_error::AppError, AppState};
use axum::{
    extract::{Query, State},
    Json,
};
use log::debug;
use reqwest::StatusCode;

#[utoipa::path(get, path = "/api/debug/subreddit_about", responses(), params())]
pub async fn subreddit_about(
    State(mut state): State<AppState>,
    Query(params): Query<AnyParams>,
) -> Result<Json<SubredditAbout>, AppError> {
    let subreddit = params
        .get("r")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `subreddit` parameter"))?;
    let req = SubredditAboutRequest {
        subreddit: subreddit.to_string(),
    };
    let about = state.fetcher.fetch_about::<SubredditAbout>(req).await?;

    Ok(Json(about))
}

#[utoipa::path(get, path = "/api/debug/post_comments", responses(), params())]
pub async fn post_comments(
    State(mut state): State<AppState>,
    user_info: GoogleUserInfo,
) -> Result<ReportAck, AppError> {
    let request = FetcherFeedRequest {
        resource_kind: RedditFeedKind::PostComments,
        report_types: vec![RMoodsReportType::Sarcasm],
        data_sources: vec![DataSource {
            name: "interestg".to_string(),
            post_id: Some("1g7e1g6".to_string()),
            share: 1.0,
        }],
        size: RequestSize::Custom(10),
        sorting: FeedSorting::New,
    };
    let requests_to_make = u16::from(request.size.clone());

    let make_report = async move {
        let (mut data, _) = state.fetcher.fetch_feed::<PostComments>(request).await?;

        let more_comments = state
            .fetcher
            .fetch_more_comments(&data.more, requests_to_make)
            .await?;

        data.list.extend(more_comments);
        data.more.clear();
        Ok::<PostComments, FetcherError>(data)
    };

    tokio::spawn(async move {
        match make_report.await {
            Ok(report) => {
                log::info!("Returning {} post comments", report.list.len());
                state
                    .system_tx
                    .send(SystemMessage::ReportDone((Box::new(report), user_info)))
                    .await
                    .unwrap();
            }
            Err(e) => {
                log::error!("Failed to make report: {:?}", e);
                state
                    .system_tx
                    .send(ReportError((AppError::from(e), user_info)))
                    .await
                    .unwrap();
            }
        }
    });

    Ok(ReportAck::new())
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
    let about = state.fetcher.fetch_about::<UserAbout>(req).await?;
    Ok(Json(about))
}

// TODO: Add proper response type for acknowledged requests
#[utoipa::path(get, path = "/api/debug/subreddit_posts", responses(), params())]
pub async fn subreddit_posts(
    State(mut state): State<AppState>,
    user_info: GoogleUserInfo,
) -> Result<ReportAck, AppError> {
    let request = FetcherFeedRequest {
        resource_kind: RedditFeedKind::PostComments,
        report_types: vec![RMoodsReportType::Sarcasm],
        data_sources: vec![DataSource {
            name: "nosleeping1213".to_string(),
            post_id: None,
            share: 1.0,
        }],
        size: RequestSize::Custom(30),
        sorting: FeedSorting::New,
    };

    let make_report = async move {
        let (data, _) = state.fetcher.fetch_feed::<Posts>(request).await?;
        Ok::<Posts, FetcherError>(data)
    };

    tokio::spawn(async move {
        log::info!("Spawning a new task to fetch subreddit posts");
        match make_report.await {
            Ok(data) => {
                log::debug!("Returning {} subreddit posts", data.list.len());
                state
                    .system_tx
                    .send(SystemMessage::ReportDone((Box::new(data), user_info)))
                    .await
                    .unwrap();
                log::debug!("Sent the subreddit posts to the WebSocket service");
            }
            Err(e) => {
                log::error!("Failed to make report: {:?}", e);
                state
                    .system_tx
                    .send(ReportError((AppError::from(e), user_info)))
                    .await
                    .unwrap();
            }
        }
    });

    Ok(ReportAck::new())
}

#[utoipa::path(get, path = "/api/debug/user_posts", responses(), params())]
pub async fn user_posts(
    State(mut state): State<AppState>,
    user_info: GoogleUserInfo,
) -> Result<ReportAck, AppError> {
    let request = FetcherFeedRequest {
        resource_kind: RedditFeedKind::UserPosts,
        report_types: vec![RMoodsReportType::Sarcasm],
        data_sources: vec![DataSource {
            name: "spezusususdsad".to_string(),
            post_id: None,
            share: 1.0,
        }],
        size: RequestSize::Custom(10),
        sorting: Default::default(),
    };

    // TODO: If there are no requests, return appropriate message to the user

    let make_report = async move {
        let (data, _) = state.fetcher.fetch_feed::<UserPosts>(request).await?;
        Ok::<UserPosts, FetcherError>(data)
    };

    tokio::spawn(async move {
        log::info!("Spawning a new task to fetch user posts");
        match make_report.await {
            Ok(data) => {
                log::debug!("Returning {} user posts", data.posts.len());
                log::debug!("Returning {} user comments", data.comments.len());
                state
                    .system_tx
                    .send(SystemMessage::ReportDone((Box::new(data), user_info)))
                    .await
                    .unwrap();
                log::debug!("Sent the subreddit posts to the WebSocket service");
            }
            Err(e) => {
                log::error!("Failed to make report: {:?}", e);
                state
                    .system_tx
                    .send(ReportError((AppError::from(e), user_info)))
                    .await
                    .unwrap();
            }
        }
    });

    Ok(ReportAck::new())
}
