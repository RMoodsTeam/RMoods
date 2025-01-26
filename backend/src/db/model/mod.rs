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
#[derive(sqlx::FromRow, Debug)]
#[sqlx(type_name = "report_record")]
pub(super) struct DbReport {
    pub(super) id: ReportId,
    //
    #[sqlx(rename = "google_id")]
    pub(super) user_id: GoogleId,
    pub(super) title: String,
    pub(super) description: String,
    pub(super) is_public: bool,
    pub(super) is_successful: bool,
    pub(super) is_in_progress: bool,
    pub(super) is_error: bool,
    pub(super) error_message: Option<String>,
    //
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,

    /// # WARNING
    /// This is a side effect of a crappy, leaky abstraction.
    /// This field is not part of the original [Report](crate::report::report::Report) struct.
    /// It's here to make it easier to calculate the number of pages that will be available for a given [ReportQuery](crate::report::report_query::ReportQuery).
    ///
    /// This field is not to be used in any other context.
    /// It's only ever used in the [Report::get_by_query](crate::report::report::Report::get_by_query) method.
    /// Optional so that the report can be fetched without this field present in DB rows.
    pub(crate) total_reports: Option<i32>,
}

#[derive(sqlx::FromRow, Debug)]
pub(super) struct DbDataRequest {
    pub(super) id: Uuid,
    pub(super) report_id: ReportId,
    //
    pub(super) feed_kind: String,
    pub(super) size: i32,
    pub(super) sort_by_kind: String,
    pub(super) sort_by_time: Option<String>,
    //
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

pub(super) struct DbDataSource {
    pub(super) id: Uuid,
    pub(super) data_request_id: Uuid,
    //
    pub(super) name: String,
    pub(super) post_id: Option<String>,
    pub(super) share: i32,
    //
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

/// Represents a singular [NlpResponse](crate::nlp::nlp_response::NlpResponse) analysis of some kind.
#[derive(sqlx::FromRow, Debug)]
pub(super) struct DbNlpAnalysis {
    pub(super) id: Uuid,
    pub(super) report_id: ReportId,
    //
    pub(super) kind: String,
    pub(super) generated_in: f64,
    pub(super) analysis: Value,
    //
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}
