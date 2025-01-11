use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Response from the NLP service for some analysis.
/// Preserves the order of the input texts.
/// Iterate in lockstep over the two fields to get the corresponding values.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NlpResponse {
    /// Classification labels. Indicates the result of the analysis.
    pub labels: Vec<String>,
    /// Confidence scores for each label. Higher is more confident.
    pub confidences: Vec<f64>,
}
