use crate::env::NLP_URL;
use crate::nlp::error::NlpError;
use crate::nlp::nlp_response::NlpAnalysis;
use log_derive::logfn;
use serde_with::serde_derive::Serialize;

#[derive(Debug, Clone)]
pub struct NlpClient {
    nlp_url: String,
    pub http: reqwest::Client,
}

#[derive(Serialize, Debug)]
struct NlpRequest {
    text: Vec<String>,
}

/// Truncate a string to a maximum length.
/// If the string is shorter than the maximum length, return the original string.
fn truncate_string(s: String, max_len: usize) -> String {
    if s.len() <= max_len {
        s
    } else {
        s.chars().take(max_len).collect()
    }
}

fn truncate_inputs(input: &[String], max_len: usize) -> Vec<String> {
    input
        .iter()
        .map(|s| truncate_string(s.clone(), max_len))
        .collect()
}

impl NlpClient {
    pub fn new() -> Self {
        NlpClient {
            nlp_url: std::env::var(NLP_URL).expect("NLP_URL must be set"),
            http: reqwest::Client::new(),
        }
    }

    #[logfn(err = "ERROR", fmt = "Failed to analyze language: {0:?}")]
    pub async fn analyze_language(&self, input: &Vec<String>) -> Result<NlpAnalysis, NlpError> {
        let url = format!("{}/report/language", self.nlp_url);

        log::debug!("Handling only first 10 inputs. Truncating each input to 100 characters.");
        // TODO: Add parallel processing for large inputs, input sampling
        let request = NlpRequest {
            text: truncate_inputs(input, 100),
        };

        log::debug!("Sending request to NLP service: {:?}", request);

        let res = self.http.post(&url).json(&request).send().await?;
        Ok(res.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        let _ = env_logger::builder().is_test(true).try_init();
        std::env::set_var(NLP_URL, "http://localhost:8002");
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_language() {
        setup();
        let client = NlpClient::new();
        let input = vec![
            "Hello, world!".to_string(),
            "Bonjour, le monde!".to_string(),
        ];
        let res = client.analyze_language(&input).await.unwrap();
        dbg!(res);
    }
}
