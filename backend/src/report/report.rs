use crate::auth::user::GoogleId;
use crate::fetcher::feed_request::RedditFeedKind;
use crate::fetcher::model::post_comments::PostComments;
use crate::fetcher::model::posts::Posts;
use crate::fetcher::model::reddit_data::RedditFeedData;
use crate::fetcher::model::user_posts::UserPosts;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpAnalysis;
use crate::report::report_error::ReportError;
use crate::report::report_request::ReportRequest;
use crate::report::report_status::ReportStatus;
use crate::util::get_utc_timestamp;
use crate::AppState;
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

pub type ReportId = String;
pub type ReportAnalysesMap = HashMap<NlpAnalysisKind, NlpAnalysis>;

pub fn new_report_id() -> ReportId {
    nanoid!(10)
}

/// An RMoods report.
///
/// Based off of NLP analysis of Reddit feeds.
/// Contains metadata and a list of analyses.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Report {
    pub id: ReportId,
    /// Information about the user that requested the report.
    pub user_id: GoogleId,
    pub title: String,
    pub description: String,
    /// Whether the report is public.
    pub is_public: bool,
    pub status: ReportStatus,
    pub analyses: ReportAnalysesMap,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Report {
    pub fn empty_in_progress(request: ReportRequest, user_id: GoogleId) -> Self {
        Report {
            id: new_report_id(),
            user_id,
            title: request.title,
            description: request.description,
            is_public: request.is_public,
            status: ReportStatus::InProgress,
            analyses: ReportAnalysesMap::new(),
            created_at: get_utc_timestamp(),
            updated_at: get_utc_timestamp(),
        }
    }

    pub async fn generate_analyses(
        &self,
        request: ReportRequest,
        state: &mut AppState,
    ) -> Result<ReportAnalysesMap, ReportError> {
        let text_data = match request.data_request.resource_kind {
            RedditFeedKind::SubredditPosts => state
                .fetcher
                .fetch_feed::<Posts>(request.data_request)
                .await?
                .0
                .extract_texts(),
            RedditFeedKind::UserPosts => state
                .fetcher
                .fetch_feed::<UserPosts>(request.data_request)
                .await?
                .0
                .extract_texts(),
            RedditFeedKind::PostComments => state
                .fetcher
                .fetch_feed::<PostComments>(request.data_request)
                .await?
                .0
                .extract_texts(),
        };

        let analyses = state
            .nlp_client
            .analyze_parallel(request.nlp_request, &text_data)
            .await?;

        Ok(analyses)
    }

    pub fn fill(&mut self, analyses: ReportAnalysesMap) {
        self.analyses = analyses;
        self.status = ReportStatus::Success;
    }
}
