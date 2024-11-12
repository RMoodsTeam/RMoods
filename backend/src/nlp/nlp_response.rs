use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Private trait used to constrain the types that can be used as the inner type of NlpResponse.
/// Blanket implemented for all types that implement Serialize, DeserializeOwned, Debug, and Clone.
pub trait NlpResponseInner: Clone + Send + Serialize {}
impl<T> NlpResponseInner for T where T: Serialize + DeserializeOwned + Debug + Clone + Send {}

/// Response from the NLP service for language analysis.
/// Preserves the order of the input texts.
/// * `language` field contains the detected languages of the input texts.
/// * `predicted` field contains the predicted languages of the input texts.
///
/// Iterate in lockstep over the two fields to get the language and prediction certainty.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawLanguageResponse {
    pub language: Vec<String>,
    pub predicted: Vec<String>,
}

/// Metadata for the NLP response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NlpMetadata {
    /// Time it took to generate the response in seconds.
    pub generated_in: f64,
}

/// Generic NLP response type.
/// Contains metadata and a list of analysis results.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NlpResponse<T: NlpResponseInner> {
    pub metadata: NlpMetadata,
    pub results: Vec<T>,
}
