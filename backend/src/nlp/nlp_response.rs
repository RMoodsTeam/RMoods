use crate::nlp::analysis::NlpAnalysisKind;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Metadata for the NLP response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NlpMetadata {
    /// Time it took to generate the response in seconds.
    #[serde(rename(serialize = "generatedIn"))]
    pub generated_in: f64,
}

/// Response from the NLP service for some analysis.
/// Preserves the order of the input texts.
/// Iterate in lockstep over the two fields to get the corresponding values.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NlpResponse {
    /// Classification labels. Indicates the result of the analysis.
    pub labels: Vec<String>,
    /// Confidence scores for each label. Higher is more confident.
    pub confidences: Vec<f64>,
}

/// Represents a full NLP analysis response.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NlpAnalysis {
    /// Kind of analysis performed.
    pub kind: NlpAnalysisKind,
    /// Metadata for the response.
    pub metadata: NlpMetadata,
    /// Results of the analysis.
    pub results: Vec<NlpResponse>,
}
