use crate::api::auth::google::GoogleId;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpResponse;
use crate::nlp::report::ReportId;
use chrono::{DateTime, Utc};
use sqlx::types::{Json, Uuid};

/// Represents a [User](crate::api::auth::google::User)
#[derive(sqlx::FromRow)]
pub struct DbUser {
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

/// Represents a [Report](crate::nlp::report::Report)
#[derive(sqlx::FromRow)]
pub struct DbReport {
    id: Uuid,
    //
    display_id: ReportId,
    user_id: GoogleId,
    title: String,
    description: String,
    is_public: bool,
    metadata_id: Uuid,
    analyses_id: Uuid,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Represents [ReportMetadata](crate::nlp::report::ReportMetadata) of a report.
#[derive(sqlx::FromRow)]
pub struct DbReportMetadata {
    id: Uuid,
    //
    report_created_at: DateTime<Utc>,
    report_updated_at: DateTime<Utc>,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Represents the hashmap of analyses from a [Report](crate::nlp::report::Report).
#[derive(sqlx::FromRow)]
pub struct DbReportAnalysesMap {
    id: Uuid,
    //
    clickbait_id: Option<Uuid>,
    hate_speech_id: Option<Uuid>,
    keywords_id: Option<Uuid>,
    language_id: Option<Uuid>,
    politics_id: Option<Uuid>,
    sarcasm_id: Option<Uuid>,
    sentiment_id: Option<Uuid>,
    spam_id: Option<Uuid>,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Represents a singular [NlpResponse](crate::nlp::nlp_response::NlpResponse) analysis of some kind.
#[derive(sqlx::FromRow)]
pub struct DbNlpAnalysis {
    id: Uuid,
    //
    /// References [DbNlpMetadata]
    nlp_metadata_id: Uuid,
    kind: NlpAnalysisKind,
    analysis: Json<Vec<NlpResponse>>,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Represents [NlpMetadata](crate::nlp::nlp_response::NlpMetadata) of an analysis.
#[derive(sqlx::FromRow)]
pub struct DbNlpMetadata {
    id: Uuid,
    //
    generated_in: f64,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
