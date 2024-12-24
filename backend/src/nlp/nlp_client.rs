use crate::env::{NLP_API_KEY, NLP_URL};
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::error::NlpError;
use crate::nlp::nlp_request::NlpRequest;
use crate::nlp::nlp_response::NlpAnalysis;
use log_derive::logfn;
use serde_json::Value;
use serde_with::serde_derive::Serialize;

#[derive(Debug, Clone)]
pub struct NlpClient {
    nlp_url: String,
    api_key: String,
    pub http: reqwest::Client,
}

#[derive(Serialize, Debug)]
struct NlpServiceRequest {
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
            api_key: std::env::var(NLP_API_KEY).expect("NLP_API_KEY must be set"),
            http: reqwest::Client::new(),
        }
    }

    fn make_url(nlp_url: &str, analysis: NlpAnalysisKind) -> String {
        type A = NlpAnalysisKind;
        let endpoint = match analysis {
            A::Language => "/language",
            A::Sentiment => "/sentiment",
            A::Sarcasm => "/sarcasm",
            A::Spam => "/spam",
            A::Politics => "/politics",
            A::HateSpeech => "/hate-speech",
            A::Clickbait => "/clickbait",
            A::Keywords => "/keywords",
        };
        format!("{}{}", nlp_url, endpoint)
    }

    #[logfn(err = "ERROR", fmt = "Failed to analyze language: {0:?}")]
    pub async fn analyze(
        &self,
        kind: NlpAnalysisKind,
        input: &Vec<String>,
    ) -> Result<NlpAnalysis, NlpError> {
        let url = Self::make_url(&self.nlp_url, kind);

        log::debug!("Handling only first 10 inputs. Truncating each input to 100 characters.");
        // TODO: Add parallel processing for large inputs, input sampling
        let nlp_request = NlpServiceRequest {
            text: truncate_inputs(input, 100),
        };

        log::debug!("Sending request to NLP service: {:?}", nlp_request);

        let res = self
            .http
            .post(url)
            .json(&nlp_request)
            .header("access_token", &self.api_key)
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn analyze_parallel(
        &self,
        nlp_request: NlpRequest,
        input: &Vec<String>,
    ) -> Result<Vec<(NlpAnalysisKind, NlpAnalysis)>, NlpError> {
        let futures = nlp_request
            .analyses
            .clone()
            .into_iter()
            .map(|kind| self.analyze(kind, input))
            .collect::<Vec<_>>();

        let analyses = futures::future::join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(nlp_request
            .analyses
            .into_iter()
            .zip(analyses.into_iter())
            .collect())
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
        let res = client
            .analyze(NlpAnalysisKind::Language, &input)
            .await
            .unwrap();
        dbg!(res);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_sentiment() {
        setup();
        let client = NlpClient::new();
        let input = vec!["I love this!".to_string(), "I hate this!".to_string()];
        let res = client
            .analyze(NlpAnalysisKind::Sentiment, &input)
            .await
            .unwrap();
        dbg!(res);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_sarcasm() {
        setup();
        let client = NlpClient::new();
        let input = vec!["I love this!".to_string(), "I hate this!".to_string()];
        let res = client
            .analyze(NlpAnalysisKind::Sarcasm, &input)
            .await
            .unwrap();
        dbg!(res);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_spam() {
        setup();
        let client = NlpClient::new();
        let input = vec![
            "Click here to win a free iPhone!".to_string(),
            "Hello, world!".to_string(),
        ];
        let res = client.analyze(NlpAnalysisKind::Spam, &input).await.unwrap();
        dbg!(res);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_hate_speech() {
        setup();
        let client = NlpClient::new();
        let input = vec!["I hate you!".to_string(), "I love you!".to_string()];
        let res = client
            .analyze(NlpAnalysisKind::HateSpeech, &input)
            .await
            .unwrap();
        dbg!(res);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_clickbait() {
        setup();
        let client = NlpClient::new();
        let input = vec![
            "You won't believe what happens next!".to_string(),
            "Hello, world!".to_string(),
        ];
        let res = client
            .analyze(NlpAnalysisKind::Clickbait, &input)
            .await
            .unwrap();
        dbg!(res);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_keywords() {
        setup();
        let client = NlpClient::new();
        let input = vec![
            "Hello, world!".to_string(),
            "Bonjour, le monde!".to_string(),
        ];
        let res = client
            .analyze(NlpAnalysisKind::Keywords, &input)
            .await
            .unwrap();
        dbg!(res);
    }

    #[ignore]
    #[tokio::test]
    async fn test_get_politics() {
        setup();
        let client = NlpClient::new();
        let input = vec![
            "Democrats are great!".to_string(),
            "Republicans are great!".to_string(),
        ];
        let res = client
            .analyze(NlpAnalysisKind::Politics, &input)
            .await
            .unwrap();
        dbg!(res);
    }
}
