use crate::nlp::analysis::NlpAnalysisKind;
use crate::validation::validated::Validated;
use crate::validation::validation_error::ValidationError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NlpRequest {
    pub analyses: Vec<NlpAnalysisKind>,
}

impl Validated for NlpRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.analyses.is_empty() {
            return Err(ValidationError::Invalid(
                "NLP Request analyses cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
