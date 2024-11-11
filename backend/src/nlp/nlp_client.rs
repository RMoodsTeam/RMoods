use crate::env::NLP_URL;
use crate::nlp::nlp_response::{LanguageResponse, NlpResponse};
use log_derive::logfn;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct NlpClient {
    nlp_url: String,
    pub http: reqwest::Client,
}

impl NlpClient {
    pub fn new() -> Self {
        NlpClient {
            nlp_url: std::env::var(NLP_URL).expect("NLP_URL must be set"),
            http: reqwest::Client::new(),
        }
    }

    #[logfn(err = "ERROR", fmt = "Failed to analyze language: {0:?}")]
    pub async fn get_language(
        &self,
        input: &[String],
    ) -> anyhow::Result<NlpResponse<LanguageResponse>> {
        let url = format!("{}/report/language", self.nlp_url);

        let request = json!({
            "text": input
        });

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
        let input = vec!["Hello, world!".to_string()];
        let res = client.get_language(&input).await.unwrap();
        dbg!(res);
    }
}
