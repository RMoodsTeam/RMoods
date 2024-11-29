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
/// * `language` field contains the detected languages of the input texts.
/// * `predicted` field contains the predicted languages of the input texts.
///
/// Iterate in lockstep over the two fields to get the corresponding values.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NlpResponse {
    pub label: Vec<String>,
    pub confidence: Vec<f64>,
}

/// Generic NLP response type.
/// Contains metadata and a list of analysis results.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NlpAnalysis {
    pub kind: NlpAnalysisKind,
    pub metadata: NlpMetadata,
    pub results: Vec<NlpResponse>,
}
