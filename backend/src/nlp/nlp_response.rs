use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LanguageResponse {
    pub language: Vec<String>,
    pub predicted: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct NlpMetadata {
    pub generated_in: f64,
}

#[derive(Deserialize, Debug)]
pub struct NlpResponse<T> {
    pub metadata: NlpMetadata,
    pub results: Vec<T>,
}
