use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpResponse;
use serde::{Deserialize, Serialize};

/// Represents a full NLP analysis response.
///
/// One analysis of a given kind can be done for each report.
/// This analysis contains results for each item of the fetched data, i.e. each Reddit post/comment.
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
