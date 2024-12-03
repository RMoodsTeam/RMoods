use crate::auth::google::GoogleId;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpAnalysis;
use nanoid::nanoid;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

/// Metadata for an RMoods report.
#[derive(Debug, Clone, Serialize)]
pub struct ReportMetadata {
    /// The UNIX timestamp of the report's creation.
    pub created_at: u64,
    /// Information about the user that requested the report.
    pub user_id: GoogleId,
    /// Whether the report is public.
    pub is_public: bool,
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
pub struct RMoodsReport {
    pub id: ReportId,
    pub metadata: ReportMetadata,
    pub analyses: HashMap<NlpAnalysisKind, NlpAnalysis>,
}
