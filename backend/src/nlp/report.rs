use crate::api::auth::google::GoogleUserInfo;
use crate::nlp::nlp_response::{NlpResponse, NlpResponseInner};
use dyn_clone::DynClone;
use serde::Serialize;
use std::fmt::Debug;

/// A trait for a report that can be sent over the WebSocket.
/// Used to send reports from the main thread to the WebSocket service.
#[typetag::serialize(tag = "type")]
pub trait SendableRMoodsReport: Send + DynClone + Debug {
    fn metadata(&self) -> &ReportMetadata;
}
// Macro magic to make it possible to clone Box<dyn SendableRMoodsReport>
dyn_clone::clone_trait_object!(SendableRMoodsReport);

/// Metadata for an RMoods report.
#[derive(Debug, Clone, Serialize)]
pub struct ReportMetadata {
    /// The UNIX timestamp of the report's creation.
    pub created_at: u64, // TODO: Make private
    /// Information about the user that requested the report.
    pub user_info: GoogleUserInfo,
    /// Whether the report is public.
    pub is_public: bool,
}

/// An RMoods report.
///
/// Based off of NLP analysis of Reddit feeds.
/// Contains metadata and a raw response from the NLP service.
#[derive(Debug, Clone, Serialize)]
pub struct RMoodsReport<T: NlpResponseInner> {
    pub metadata: ReportMetadata,
    pub nlp_response: NlpResponse<T>,
}

#[typetag::serialize]
impl<T> SendableRMoodsReport for RMoodsReport<T>
where
    T: NlpResponseInner + Debug,
{
    fn metadata(&self) -> &ReportMetadata {
        &self.metadata
    }
}
