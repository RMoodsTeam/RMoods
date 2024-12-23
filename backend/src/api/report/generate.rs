use crate::api::report::report_ack::ReportAck;
use crate::app_error::AppError;
use crate::auth::google::JwtUserInfo;
use crate::auth::user::GoogleId;
use crate::db::db_stored::DbStored;
use crate::fetcher::feed_request::{FetcherDataRequest, RedditFeedKind};
use crate::fetcher::fetcher::RMoodsFetcher;
use crate::fetcher::model::post_comments::PostComments;
use crate::fetcher::model::posts::Posts;
use crate::fetcher::model::reddit_data::RedditFeedData;
use crate::fetcher::model::user_posts::UserPosts;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_client::NlpClient;
use crate::nlp::nlp_request::NlpRequest;
use crate::report::report::{new_report_id, Report, ReportAnalysesMap, ReportMetadata};
use crate::report::report_request::ReportRequest;
use crate::report::report_status::ReportStatus;
use crate::validation::validated::Validated;
use crate::websocket::SystemMessage;
use crate::websocket::SystemMessage::ReportError;
use crate::AppState;
use axum::extract::State;
use chrono::Utc;
use std::collections::HashMap;

/// Create a report from the given data.
///
/// The report is created by applying the NLP analysis to the texts extracted from the data.
pub async fn nlp_analysis<T: RedditFeedData>(
    nlp_client: &NlpClient,
    data: T,
    nlp_request: NlpRequest,
    user_id: GoogleId,
) -> Result<Report, AppError> {
    let texts = data.extract_texts();
    let analyses = nlp_client
        .analyze_parallel(nlp_request, &texts)
        .await?
        .into_iter()
        .collect::<HashMap<_, _>>();
    let report = Report {
        id: new_report_id(),
        user_id,
        title: "RMoods Report".to_string(),
        description: "An RMoods report generated from Reddit data.".to_string(),
        is_public: true,
        status: ReportStatus::InProgress,
        metadata: ReportMetadata {
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        analyses_map: ReportAnalysesMap { analyses },
    };
    Ok(report)
}

/// Performs all steps needed to create a report.
///
/// 1. Fetches the data from the Reddit API.
/// 2. Creates an NLP report from the data.
async fn generate_report<T: RedditFeedData>(
    fetcher: &mut RMoodsFetcher,
    report_request: ReportRequest,
    nlp: &NlpClient,
    user_info: &GoogleId,
) -> Result<Report, AppError> {
    let (data, _) = fetcher.fetch_feed::<T>(report_request.data_request).await?;
    let report = nlp_analysis(nlp, data, report_request.nlp_request, user_info.clone()).await?;
    Ok(report)
}

pub async fn generate_report_handler(
    State(mut state): State<AppState>,
    user_info: JwtUserInfo,
    report_request: ReportRequest,
) -> Result<ReportAck, AppError> {
    log::debug!("Validating feed request: {:?}", report_request);
    report_request.validate()?;
    log::debug!("Feed request is valid");
    log::debug!("Generating report for user: {}", user_info.id);

    tokio::spawn(async move {
        let nlp = &state.nlp_client;
        let report_res = match report_request.data_request.resource_kind {
            RedditFeedKind::UserPosts => {
                generate_report::<UserPosts>(&mut state.fetcher, report_request, nlp, &user_info.id)
                    .await
            }
            RedditFeedKind::PostComments => {
                generate_report::<PostComments>(
                    &mut state.fetcher,
                    report_request,
                    nlp,
                    &user_info.id,
                )
                .await
            }
            RedditFeedKind::SubredditPosts => {
                generate_report::<Posts>(&mut state.fetcher, report_request, nlp, &user_info.id)
                    .await
            }
        };

        match report_res {
            Ok(mut report) => {
                log::info!("Report successfully generated: {}", report.id);
                report.status = ReportStatus::Success;
                report.update(&state.db).await.unwrap();
                state
                    .system_tx
                    .send(SystemMessage::ReportDone((report.id, user_info.id)))
                    .await
                    .unwrap();
            }
            Err(e) => {
                log::error!("Error generating report: {:?}", e);
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
