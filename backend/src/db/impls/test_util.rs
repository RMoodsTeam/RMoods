#![allow(dead_code)]
// Test utilities for the database implementation.
// Allowing dead code because these functions are only used in tests.

use crate::auth::user::{GoogleId, User};
use crate::db::db_client::DbClient;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::{NlpAnalysis, NlpMetadata, NlpResponse};
use crate::report::report::{Report, ReportAnalysesMap, ReportMetadata};
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

pub(super) fn get_test_user() -> User {
    User {
        id: "12345678".to_string(),
        name: "Test User".to_string(),
        given_name: "Test".to_string(),
        family_name: Some("User".to_string()),
        picture: "https://example.com/picture".to_string(),
        email: "test@example.com".to_string(),
        email_verified: true,
    }
}

pub(super) fn get_test_report(title: String, user_id: GoogleId) -> Report {
    Report {
        id: nanoid::nanoid!(),
        user_id,
        title,
        description: "Test Description".to_string(),
        is_public: false,
        status: ReportStatus::InProgress,
        metadata: ReportMetadata {
            created_at: get_utc_timestamp(),
            updated_at: get_utc_timestamp(),
        },
        analyses_map: get_test_report_analysis_map(),
    }
}

pub(super) fn get_test_nlp_analysis(kind: NlpAnalysisKind) -> NlpAnalysis {
    NlpAnalysis {
        kind,
        results: vec![NlpResponse {
            labels: vec!["LABEL_1".to_string(), "LABEL_2".to_string()],
            confidences: vec![0.9, 0.6],
        }],
        metadata: NlpMetadata { generated_in: 0.5 },
    }
}

pub(super) fn get_test_nlp_metadata() -> NlpMetadata {
    NlpMetadata { generated_in: 0.5 }
}

pub(super) fn get_test_report_analysis_map() -> ReportAnalysesMap {
    ReportAnalysesMap {
        analyses: HashMap::from([
            (
                NlpAnalysisKind::Sentiment,
                get_test_nlp_analysis(NlpAnalysisKind::Sentiment),
            ),
            (
                NlpAnalysisKind::Politics,
                get_test_nlp_analysis(NlpAnalysisKind::Politics),
            ),
        ]),
    }
}

pub(super) fn get_test_report_metadata() -> ReportMetadata {
    ReportMetadata {
        created_at: get_utc_timestamp(),
        updated_at: get_utc_timestamp(),
    }
}
