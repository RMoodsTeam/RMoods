#![allow(dead_code)]
// Test utilities for the database implementation.
// Allowing dead code because these functions are only used in tests.

use crate::auth::user::{GoogleId, User};
use crate::db::db_client::DbClient;
use crate::fetcher::data_request::{DataSource, FetcherDataRequest, RedditFeedKind};
use crate::fetcher::reddit::request::feed_sorting::FeedSorting;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::{NlpAnalysis, NlpResponse};
use crate::report::report::Report;
use crate::report::report_status::ReportStatus;
use crate::util::get_utc_timestamp;
use std::collections::HashMap;

pub(super) async fn get_db() -> DbClient {
    dotenvy::dotenv().ok();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    DbClient::new(pool)
}

pub(super) fn get_test_user(id: String) -> User {
    User {
        id: id.clone(),
        name: id,
        given_name: "Test".to_string(),
        family_name: Some("User".to_string()),
        picture: "https://example.com/picture".to_string(),
        email: "test@example.com".to_string(),
        email_verified: true,
    }
}

pub(super) fn get_test_report(title: String, user_id: GoogleId) -> Report {
    let now = get_utc_timestamp();
    Report {
        id: nanoid::nanoid!(),
        user_id,
        title,
        description: "Test Description".to_string(),
        is_public: false,
        status: ReportStatus::InProgress,
        created_at: now,
        updated_at: now,
        analyses: get_test_report_analyses(),
        data_request: get_test_data_request(),
    }
}

pub(super) fn get_test_data_request() -> FetcherDataRequest {
    FetcherDataRequest {
        feed_kind: RedditFeedKind::SubredditPosts,
        data_sources: vec![DataSource {
            name: "r/askreddit".to_string(),
            post_id: None,
            share: 100,
        }],
        size: 10,
        sort_by: FeedSorting::Hot,
    }
}

pub(super) fn get_test_nlp_analysis(kind: NlpAnalysisKind) -> NlpAnalysis {
    NlpAnalysis {
        kind,
        results: vec![NlpResponse {
            labels: vec!["LABEL_1".to_string(), "LABEL_2".to_string()],
            confidences: vec![0.9, 0.6],
        }],
        generated_in: 0.5,
    }
}

pub(super) fn get_test_report_analyses() -> HashMap<NlpAnalysisKind, NlpAnalysis> {
    HashMap::from([
        (
            NlpAnalysisKind::Sentiment,
            get_test_nlp_analysis(NlpAnalysisKind::Sentiment),
        ),
        (
            NlpAnalysisKind::Politics,
            get_test_nlp_analysis(NlpAnalysisKind::Politics),
        ),
    ])
}
