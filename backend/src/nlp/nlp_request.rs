use crate::nlp::analysis::NlpAnalysisKind;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NlpRequest {
    pub analyses: Vec<NlpAnalysisKind>,
}
