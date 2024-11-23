use crate::api::auth::google::GoogleId;
use crate::nlp::nlp_response::{NlpResponse, RawLanguageResponse};
use nanoid::nanoid;
use serde::Serialize;
use std::fmt::Debug;
// /// A trait for a report that can be sent over the WebSocket.
// /// Used to send reports from the main thread to the WebSocket service.
// #[typetag::serialize(tag = "type")]
// pub trait SendableRMoodsReport: Send + DynClone + Debug {
//     fn metadata(&self) -> &ReportMetadata;
// }
// // Macro magic to make it possible to clone Box<dyn SendableRMoodsReport>
// dyn_clone::clone_trait_object!(SendableRMoodsReport);

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
    // let alphabet_str = "1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // let alphabet = alphabet_str.chars().collect::<Vec<char>>();
    nanoid!(10)
}

/// An RMoods report.
///
/// Based off of NLP analysis of Reddit feeds.
/// Contains metadata and a raw response from the NLP service.
#[derive(Serialize)]
pub struct RMoodsReport {
    pub id: ReportId,
    pub metadata: ReportMetadata,
    pub analyses: NlpAnalyses,
}

#[derive(Serialize)]
pub struct NlpAnalyses {
    pub language: Option<NlpResponse<RawLanguageResponse>>,
}
