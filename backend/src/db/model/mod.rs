use crate::api::auth::google::GoogleId;
use crate::nlp::nlp_response::NlpResponse;
use crate::nlp::report::ReportId;
use chrono::{DateTime, Utc};
use sqlx::types::{Json, Uuid};

/// Represents a [Report](crate::nlp::report::Report)
#[derive(sqlx::FromRow)]
struct DbReport {
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
struct DbReportMetadata {
    id: Uuid,
    report_id: Uuid,
    //
    report_created_at: DateTime<Utc>,
    report_updated_at: DateTime<Utc>,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Represents the hashmap of analyses from a [Report](crate::nlp::report::Report).
#[derive(sqlx::FromRow)]
struct DbReportAnalyses {
    id: Uuid,
    report_id: Uuid,
    //
    clickbait: Option<Uuid>,
    hate_speech: Option<Uuid>,
    keywords: Option<Uuid>,
    language: Option<Uuid>,
    politics: Option<Uuid>,
    sarcasm: Option<Uuid>,
    sentiment: Option<Uuid>,
    spam: Option<Uuid>,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Represents a singular [NlpResponse](crate::nlp::nlp_response::NlpResponse) analysis of some kind.
#[derive(sqlx::FromRow)]
struct DbReportAnalysis {
    id: Uuid,
    /// References [DbNlpMetadata]
    nlp_metadata_id: Uuid,
    //
    kind: String,
    analysis: Json<Vec<NlpResponse>>,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Represents [NlpMetadata](crate::nlp::nlp_response::NlpMetadata) of an analysis.
#[derive(sqlx::FromRow)]
struct DbNlpMetadata {
    id: Uuid,
    analysis_id: Uuid,
    //
    generated_in: f64,
    //
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
