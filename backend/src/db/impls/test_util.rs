#![allow(dead_code)]
// Test utilities for the database implementation.
// Allowing dead code because these functions are only used in tests.

use crate::auth::user::{GoogleId, User};
use crate::report::report::{Report, ReportAnalysesMap, ReportMetadata};
use crate::report::report_status::ReportStatus;
use chrono::Utc;
use std::collections::HashMap;

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
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        analyses_map: ReportAnalysesMap {
            analyses: HashMap::new(),
        },
    }
}
