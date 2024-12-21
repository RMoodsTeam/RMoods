use crate::auth::user::GoogleId;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpAnalysis;
use crate::report::report_error::ReportError;
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

#[derive(Serialize)]
pub enum ReportStatus {
    Success,
    InProgress,
    Error(String),
}

impl ReportStatus {
    pub fn from_booleans(
        success: bool,
        in_progress: bool,
        error: bool,
        error_msg: Option<String>,
    ) -> Result<Self, ReportError> {
        match (success, in_progress, error, error_msg) {
            (true, _, _, _) => Ok(Self::Success),
            (_, true, _, _) => Ok(Self::InProgress),
            (_, _, true, Some(msg)) => Ok(Self::Error(msg)),
            (_, _, _, _) => Err(ReportError::InvalidState(
                "Invalid report status".to_string(),
            )),
        }
    }

    pub fn is_successful(&self) -> bool {
        matches!(self, Self::Success)
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error(msg) => Some(msg),
            _ => None,
        }
    }
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
    pub status: ReportStatus,
    pub metadata: ReportMetadata,
    pub analyses_map: ReportAnalysesMap,
}
