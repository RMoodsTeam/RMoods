use crate::api::auth::google::{GoogleId, JwtUserInfo};
use crate::api::report::report_ack::ReportAck;
use crate::app_error::AppError;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_client::NlpClient;
use crate::nlp::report::{new_report_id, RMoodsReport, ReportMetadata};
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
use jsonwebtoken::get_current_timestamp;
use std::collections::HashMap;

/// Create a report from the given data.
///
/// The report is created by applying the NLP analysis to the texts extracted from the data.
pub async fn nlp_analysis<T: RedditFeedData>(
    nlp_client: &NlpClient,
    data: T,
    user_id: GoogleId,
) -> Result<RMoodsReport, AppError> {
    let texts = data.extract_texts();
    let language_analysis = nlp_client.analyze_language(&texts).await?;
    let report = RMoodsReport {
        id: new_report_id(),
        metadata: ReportMetadata {
            created_at: get_current_timestamp(),
            user_id,
            is_public: true,
        },
        analyses: HashMap::from([(NlpAnalysisKind::Language, language_analysis)]),
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
    user_info: &GoogleId,
) -> Result<RMoodsReport, AppError> {
    let (data, _) = fetcher.fetch_feed::<T>(feed_request).await?;
    let report = nlp_analysis(nlp, data, user_info.clone()).await?;
    Ok(report)
}

pub async fn generate_report_handler(
    State(mut state): State<AppState>,
    user_info: JwtUserInfo,
    feed_request: FetcherFeedRequest,
) -> Result<ReportAck, AppError> {
    log::debug!("Validating feed request: {:?}", feed_request);
    feed_request.validate()?;
    log::debug!("Feed request is valid");
    log::debug!("Generating report for user: {}", user_info.id);

    tokio::spawn(async move {
        let nlp = &state.nlp_client;
        let report_res = match feed_request.resource_kind {
            RedditFeedKind::UserPosts => {
                generate_report::<UserPosts>(&mut state.fetcher, feed_request, nlp, &user_info.id)
                    .await
            }
            RedditFeedKind::PostComments => {
                generate_report::<PostComments>(
                    &mut state.fetcher,
                    feed_request,
                    nlp,
                    &user_info.id,
                )
                .await
            }
            RedditFeedKind::SubredditPosts => {
                generate_report::<Posts>(&mut state.fetcher, feed_request, nlp, &user_info.id).await
            }
        };

        match report_res {
            Ok(report) => {
                state
                    .system_tx
                    .send(SystemMessage::ReportDone((report.id, user_info.id)))
                    .await
                    .unwrap();
            }
            Err(e) => {
                state
                    .system_tx
                    .send(ReportError((AppError::from(e), user_info.id)))
                    .await
                    .unwrap();
            }
        }
    });

    Ok(ReportAck::new())
}
