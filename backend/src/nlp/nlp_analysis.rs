use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpResponse;
use serde::{Deserialize, Serialize};

/// Represents a full NLP analysis response.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NlpAnalysis {
    /// Kind of analysis performed.
    pub kind: NlpAnalysisKind,
    /// Generation time for all the responses combined.
    pub generated_in: f64,
    /// Results of the analysis.
    pub results: Vec<NlpResponse>,
}
