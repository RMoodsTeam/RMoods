// use crate::api::auth::google::GoogleUserInfo;
// use crate::api::report::generate;
// use crate::api::report::report_ack::ReportAck;
// use crate::nlp::nlp_response::RawLanguageResponse;
// use crate::nlp::report::{RMoodsReport, ReportMetadata};
// use crate::reddit_fetcher::feed_request::{
//     DataSource, FetcherFeedRequest, RMoodsReportType, RedditFeedKind, RequestSize,
// };
// use crate::reddit_fetcher::model::post_comments::PostComments;
// use crate::reddit_fetcher::model::posts::Posts;
// use crate::reddit_fetcher::model::reddit_data::RedditFeedData;
// use crate::reddit_fetcher::model::user_posts::UserPosts;
// use crate::reddit_fetcher::reddit::request::params::FeedSorting;
// use crate::websocket::SystemMessage;
// use crate::websocket::SystemMessage::ReportError;
// use crate::{app_error::AppError, AppState};
// use axum::extract::State;
// use jsonwebtoken::get_current_timestamp;
//
// #[utoipa::path(get, path = "/api/debug/post_comments", responses(), params())]
// pub async fn post_comments(
//     State(mut state): State<AppState>,
//     user_info: GoogleUserInfo,
// ) -> Result<ReportAck, AppError> {
//     let request = FetcherFeedRequest {
//         resource_kind: RedditFeedKind::PostComments,
//         report_types: vec![RMoodsReportType::Sarcasm],
//         data_sources: vec![DataSource {
//             name: "interesting23".to_string(),
//             post_id: Some("1g7e1g6".to_string()),
//             share: 1.0,
//         }],
//         size: RequestSize::Custom(10),
//         sorting: FeedSorting::New,
//     };
//     let requests_to_make = u16::from(request.size.clone());
//
//     let (mut data, _) = state.fetcher.fetch_feed::<PostComments>(request).await?;
//     let more_comments = state
//         .fetcher
//         .fetch_more_comments(&data.more, requests_to_make)
//         .await?;
//
//     data.list.extend(more_comments);
//     data.more.clear();
//
//     log::debug!("Fetched {} post comments", data.list.len());
//
//     let report_res =
//         generate::nlp_analysis::<PostComments>(&mut state, data, user_info.clone()).await;
//
//     tokio::spawn(async move {
//         match report_res {
//             Ok(report) => {
//                 state
//                     .system_tx
//                     .send(SystemMessage::ReportDone(Box::new(report)))
//                     .await
//                     .unwrap();
//             }
//             Err(e) => {
//                 state
//                     .system_tx
//                     .send(ReportError((AppError::from(e), user_info)))
//                     .await
//                     .unwrap();
//             }
//         }
//     });
//
//     Ok(ReportAck::new())
// }
//
// // TODO: Add proper response type for acknowledged requests
// #[utoipa::path(get, path = "/api/debug/subreddit_posts", responses(), params())]
// pub async fn subreddit_posts(
//     State(mut state): State<AppState>,
//     user_info: GoogleUserInfo,
// ) -> Result<ReportAck, AppError> {
//     let request = FetcherFeedRequest {
//         resource_kind: RedditFeedKind::PostComments,
//         report_types: vec![RMoodsReportType::Sarcasm],
//         data_sources: vec![DataSource {
//             name: "nosleep".to_string(),
//             post_id: None,
//             share: 1.0,
//         }],
//         size: RequestSize::Custom(30),
//         sorting: FeedSorting::New,
//     };
//
//     let (data, _) = state.fetcher.fetch_feed::<Posts>(request).await?;
//     log::debug!("Fetched {} subreddit posts", data.list.len());
//     let report_res = generate::nlp_analysis::<Posts>(&mut state, data, user_info.clone()).await;
//
//     tokio::spawn(async move {
//         match report_res {
//             Ok(analysis) => {
//                 state
//                     .system_tx
//                     .send(SystemMessage::ReportDone(Box::new(analysis)))
//                     .await
//                     .unwrap();
//             }
//             Err(e) => {
//                 state
//                     .system_tx
//                     .send(ReportError((AppError::from(e), user_info)))
//                     .await
//                     .unwrap();
//             }
//         }
//     });
//
//     Ok(ReportAck::new())
// }
//
// #[utoipa::path(get, path = "/api/debug/user_posts", responses(), params())]
// pub async fn user_posts(
//     State(mut state): State<AppState>,
//     user_info: GoogleUserInfo,
// ) -> Result<ReportAck, AppError> {
//     let request = FetcherFeedRequest {
//         resource_kind: RedditFeedKind::UserPosts,
//         report_types: vec![RMoodsReportType::Sarcasm],
//         data_sources: vec![DataSource {
//             name: "spez".to_string(),
//             post_id: None,
//             share: 1.0,
//         }],
//         size: RequestSize::Custom(10),
//         sorting: Default::default(),
//     };
//
//     // TODO: If there are no requests, return appropriate message to the user
//
//     let data = state.fetcher.fetch_feed::<UserPosts>(request).await?.0;
//     log::debug!("Fetched {} user posts", data.posts.len());
//     log::debug!("Fetched {} user comments", data.comments.len());
//     let report_res = generate::nlp_analysis::<UserPosts>(&mut state, data, user_info.clone()).await;
//
//     Ok(ReportAck::new())
// }
