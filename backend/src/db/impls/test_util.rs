#![allow(dead_code)]
// Test utilities for the database implementation.
// Allowing dead code because these functions are only used in tests.

use crate::auth::user::{GoogleId, User};
use crate::db::db_client::DbClient;
use crate::db::db_stored::DbStored;
use crate::fetcher::data_request::{DataSource, FetcherDataRequest, RedditFeedKind};
use crate::fetcher::reddit::request::feed_sorting::FeedSorting;
use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::nlp_analysis::NlpAnalysis;
use crate::nlp::nlp_response::NlpResponse;
use crate::report::report::Report;
use crate::report::report_status::ReportStatus;
use crate::util::get_utc_timestamp;
use chrono::NaiveTime;
use nanoid::nanoid;
use sqlx::PgPool;
use std::collections::HashMap;
use std::ops::Sub;

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
        analyses: Some(get_test_report_analyses()),
        data_request: get_test_data_request(),
        processed_analyses: None,
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
            label: "LABEL_1".to_string(),
            confidence: 0.9,
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

fn get_random_string(len: usize) -> String {
    nanoid::nanoid!(len)
}

fn get_random_bool() -> bool {
    rand::random()
}

fn get_random_report_status() -> ReportStatus {
    match rand::random::<u8>() % 3 {
        0 => ReportStatus::InProgress,
        1 => ReportStatus::Success,
        2 => ReportStatus::Error(nanoid::nanoid!()),
        _ => panic!("Impossible value"),
    }
}

fn get_random_feed_kind() -> RedditFeedKind {
    match rand::random::<u8>() % 3 {
        0 => RedditFeedKind::SubredditPosts,
        1 => RedditFeedKind::UserPosts,
        2 => RedditFeedKind::PostComments,
        _ => panic!("Impossible value"),
    }
}

fn get_random_data_request() -> FetcherDataRequest {
    FetcherDataRequest {
        feed_kind: get_random_feed_kind(),
        data_sources: vec![DataSource {
            name: nanoid!(),
            post_id: if rand::random() {
                Some(nanoid!())
            } else {
                None
            },
            share: rand::random(),
        }],
        size: rand::random(),
        sort_by: FeedSorting::Hot,
    }
}

fn get_random_nlp_analysis_kind() -> NlpAnalysisKind {
    match rand::random::<u8>() % 8 {
        0 => NlpAnalysisKind::Sentiment,
        1 => NlpAnalysisKind::Clickbait,
        2 => NlpAnalysisKind::Politics,
        3 => NlpAnalysisKind::HateSpeech,
        4 => NlpAnalysisKind::Sarcasm,
        5 => NlpAnalysisKind::Spam,
        6 => NlpAnalysisKind::Language,
        7 => NlpAnalysisKind::Llm,
        _ => panic!("Impossible value"),
    }
}

fn get_random_nlp_analyses() -> HashMap<NlpAnalysisKind, NlpAnalysis> {
    let mut analyses = HashMap::new();
    for _ in 0..rand::random::<u8>() % 5 {
        analyses.insert(
            get_random_nlp_analysis_kind(),
            get_test_nlp_analysis(get_random_nlp_analysis_kind()),
        );
    }
    analyses
}

pub(super) async fn insert_test_reports() {
    dotenvy::dotenv().ok();
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let db = DbClient::new(pool);

    let mut time = get_utc_timestamp()
        .sub(chrono::Duration::hours(24))
        .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .unwrap();
    let end_time = time + chrono::Duration::days(1);

    while time < end_time {
        let user_id = nanoid!();
        let user = get_test_user(user_id.clone());
        let report = Report {
            id: nanoid::nanoid!(),
            user_id,
            title: get_random_string(10),
            description: get_random_string(20),
            is_public: get_random_bool(),
            status: get_random_report_status(),
            created_at: time,
            updated_at: time,
            analyses: Some(get_random_nlp_analyses()),
            data_request: get_random_data_request(),
            processed_analyses: None,
        };

        user.save(&db).await.unwrap();
        report.save(&db).await.unwrap();

        time = time + chrono::Duration::minutes(5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    // Allows for manual triggering of the inserting
    async fn manual_test_insert_test_reports() {
        insert_test_reports().await;
    }
}
