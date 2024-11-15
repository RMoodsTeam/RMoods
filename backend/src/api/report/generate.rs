use crate::api::auth::google::GoogleUserInfo;
use crate::api::report::report_ack::ReportAck;
use crate::app_error::AppError;
use crate::nlp::nlp_client::NlpClient;
use crate::nlp::nlp_response::RawLanguageResponse;
use crate::nlp::report::{RMoodsReport, ReportMetadata, SendableRMoodsReport};
use crate::reddit_fetcher::feed_request::{FetcherFeedRequest, RedditFeedKind};
use crate::reddit_fetcher::fetcher::RMoodsFetcher;
use crate::reddit_fetcher::model::post_comments::PostComments;
use crate::reddit_fetcher::model::posts::Posts;
use crate::reddit_fetcher::model::reddit_data::RedditFeedData;
use crate::reddit_fetcher::model::user_posts::UserPosts;
use crate::websocket::SystemMessage;
use crate::websocket::SystemMessage::ReportError;
use crate::AppState;
use axum::extract::State;
use axum::{debug_handler, Json};
use jsonwebtoken::get_current_timestamp;

/// Create a report from the given data.
///
/// The report is created by applying the NLP analysis to the texts extracted from the data.
pub async fn nlp_analysis<T: RedditFeedData>(
    nlp_client: &NlpClient,
    data: T,
    user_info: GoogleUserInfo,
) -> Result<RMoodsReport<RawLanguageResponse>, AppError> {
    let texts = data.extract_texts();
    let language_analysis = nlp_client.analyze_language(&texts).await?;
    let report = RMoodsReport {
        metadata: ReportMetadata {
            created_at: get_current_timestamp(),
            user_info: user_info.clone(),
            is_public: true,
        },
        nlp_response: language_analysis,
    };
    Ok(report)
}

/// Performs all steps needed to create a report.
///
/// 1. Fetches the data from the Reddit API.
/// 2. Creates an NLP report from the data.
async fn generate_report<T: RedditFeedData>(
    fetcher: &mut RMoodsFetcher,
    feed_request: FetcherFeedRequest,
    nlp: &NlpClient,
    user_info: &GoogleUserInfo,
) -> Result<Box<dyn SendableRMoodsReport>, AppError> {
    let (data, _) = fetcher.fetch_feed::<T>(feed_request).await?;
    let report = nlp_analysis(nlp, data, user_info.clone()).await?;
    Ok(Box::new(report))
}

#[debug_handler]
pub async fn generate_report_handler(
    State(mut state): State<AppState>,
    user_info: GoogleUserInfo,
    Json(feed_request): Json<FetcherFeedRequest>,
) -> Result<ReportAck, AppError> {
    dbg!(&user_info);
    dbg!(&feed_request);

    tokio::spawn(async move {
        let nlp = &state.nlp_client;
        let report_res = match feed_request.resource_kind {
            RedditFeedKind::UserPosts => {
                generate_report::<UserPosts>(&mut state.fetcher, feed_request, nlp, &user_info)
                    .await
            }
            RedditFeedKind::PostComments => {
                generate_report::<PostComments>(&mut state.fetcher, feed_request, nlp, &user_info)
                    .await
            }
            RedditFeedKind::SubredditPosts => {
                generate_report::<Posts>(&mut state.fetcher, feed_request, nlp, &user_info).await
            }
        };

        match report_res {
            Ok(report) => {
                state
                    .system_tx
                    .send(SystemMessage::ReportDone(report))
                    .await
                    .unwrap();
            }
            Err(e) => {
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
