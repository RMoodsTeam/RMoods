use crate::auth::user::GoogleId;
use crate::fetcher::data_request::{FetcherDataRequest, RedditFeedKind};
use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::into_text_data::ToTextData;
use crate::nlp::nlp_analysis::NlpAnalysis;
use crate::report::nlp_processed::ProcessedNlpAnalysis;
use crate::report::reddit_data_container::RedditDataContainer;
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
    pub processed_analyses: Option<ProcessedNlpAnalysis>,
    pub data_request: FetcherDataRequest,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Report {
    pub fn empty_in_progress(request: ReportRequest, user_id: GoogleId) -> Self {
        let now = get_utc_timestamp();
        Report {
            id: new_report_id(),
            user_id,
            title: request.title,
            description: request.description,
            is_public: request.is_public,
            status: ReportStatus::InProgress,
            analyses: ReportAnalysesMap::new(),
            processed_analyses: None,
            data_request: request.data_request,
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn generate_analyses(
        &self,
        request: ReportRequest,
        state: &mut AppState,
    ) -> Result<ReportAnalysesMap, ReportError> {
        let data = match request.data_request.feed_kind {
            RedditFeedKind::SubredditPosts => RedditDataContainer::SubredditPosts(
                state.fetcher.fetch_feed(request.data_request).await?.0,
            ),
            RedditFeedKind::PostComments => RedditDataContainer::PostComments(
                state.fetcher.fetch_feed(request.data_request).await?.0,
            ),
            RedditFeedKind::UserPosts => RedditDataContainer::UserPosts(
                state.fetcher.fetch_feed(request.data_request).await?.0,
            ),
        };

        let text_data = data.to_text_data();

        let analyses = state
            .nlp_client
            .analyze_parallel(request.nlp_request, &text_data)
            .await?;

        for (_, analysis) in &analyses {
            let items = data.values().unwrap();
            let processed = ProcessedNlpAnalysis::generate_from_analyses_map(analysis, items);
            dbg!(processed);
        }

        Ok(analyses)
    }

    pub fn fill(&mut self, analyses: ReportAnalysesMap) {
        self.analyses = analyses;
        self.status = ReportStatus::Success;
    }
}
