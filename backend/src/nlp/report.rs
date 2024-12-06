use crate::api::auth::google::GoogleId;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpAnalysis;
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

/// Metadata for an RMoods report.
#[derive(Debug, Clone, Serialize)]
pub struct ReportMetadata {
    /// The UNIX timestamp of the report's creation.
    pub created_at: DateTime<Utc>,
    /// The UNIX timestamp of the report's last update.
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct ReportAnalysesMap {
    pub analyses: HashMap<NlpAnalysisKind, NlpAnalysis>,
}

pub type ReportId = String;

pub fn new_report_id() -> ReportId {
    nanoid!(10)
}

/// An RMoods report.
///
/// Based off of NLP analysis of Reddit feeds.
/// Contains metadata and a list of analyses.
#[derive(Serialize)]
pub struct Report {
    pub id: ReportId,
    /// Information about the user that requested the report.
    pub user_id: GoogleId,
    pub title: String,
    pub description: String,
    /// Whether the report is public.
    pub is_public: bool,
    pub metadata: ReportMetadata,
    pub analyses_map: ReportAnalysesMap,
}
