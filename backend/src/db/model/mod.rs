use crate::auth::user::GoogleId;
use crate::report::report::ReportId;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::types::Uuid;

/// Represents a [User](crate::api::auth::google::User)
#[derive(sqlx::FromRow)]
pub(super) struct DbUser {
    id: Uuid,
    //
    google_id: GoogleId,
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
    email: String,
    email_verified: bool,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Represents a [Report](crate::report::report::Report)
#[derive(sqlx::FromRow)]
pub(super) struct DbReport {
    pub(super) id: Uuid,
    //
    pub(super) display_id: ReportId,
    pub(super) user_id: GoogleId,
    pub(super) title: String,
    pub(super) description: String,
    pub(super) is_public: bool,
    pub(super) is_successful: bool,
    pub(super) is_in_progress: bool,
    pub(super) is_error: bool,
    pub(super) error_message: Option<String>,
    pub(super) metadata_id: Uuid,
    pub(super) analyses_map_id: Uuid,
    //
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

/// Represents [ReportMetadata](crate::report::report::ReportMetadata) of a report.
#[derive(sqlx::FromRow)]
pub(super) struct DbReportMetadata {
    pub(super) id: Uuid,
    //
    pub(super) report_created_at: DateTime<Utc>,
    pub(super) report_updated_at: DateTime<Utc>,
    //
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

/// Represents the hashmap of analyses from a [Report](crate::report::report::Report).
#[derive(sqlx::FromRow)]
pub(super) struct DbReportAnalysesMap {
    pub(super) id: Uuid,
    //
    pub(super) clickbait_id: Option<Uuid>,
    pub(super) hate_speech_id: Option<Uuid>,
    pub(super) keywords_id: Option<Uuid>,
    pub(super) language_id: Option<Uuid>,
    pub(super) politics_id: Option<Uuid>,
    pub(super) sarcasm_id: Option<Uuid>,
    pub(super) sentiment_id: Option<Uuid>,
    pub(super) spam_id: Option<Uuid>,
    //
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

/// Represents a singular [NlpResponse](crate::nlp::nlp_response::NlpResponse) analysis of some kind.
#[derive(sqlx::FromRow)]
pub(super) struct DbNlpAnalysis {
    pub(super) id: Uuid,
    //
    /// References [DbNlpMetadata]
    pub(super) nlp_metadata_id: Uuid,
    pub(super) kind: String,
    pub(super) analysis: Value,
    //
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

/// Represents [NlpMetadata](crate::nlp::nlp_response::NlpMetadata) of an analysis.
#[derive(sqlx::FromRow)]
pub(super) struct DbNlpMetadata {
    pub(super) id: Uuid,
    //
    pub(super) generated_in: f64,
    //
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}
