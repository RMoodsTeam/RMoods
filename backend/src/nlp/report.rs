use crate::api::auth::google::GoogleUserInfo;
use crate::nlp::nlp_response::{NlpResponse, NlpResponseInner};
use dyn_clone::DynClone;
use serde::Serialize;
use std::fmt::Debug;

#[typetag::serialize(tag = "type")]
pub trait SendableRMoodsReport: Send + DynClone + Debug {
    fn metadata(&self) -> &ReportMetadata;
}
dyn_clone::clone_trait_object!(SendableRMoodsReport);

/// Temporary solution until we add proper report result structs.
/// TODO: Add proper report result structs.

#[derive(Debug, Clone, Serialize)]
pub struct ReportMetadata {
    pub created_at: u64, // TODO: Make private
    pub user_info: GoogleUserInfo,
}

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
