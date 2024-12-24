use serde::{Deserialize, Serialize};

/// The kind of analysis to perform.
/// This is used to determine the endpoint to hit on the NLP service.
/// The NLP service will return a response with the same kind of analysis.
///
/// Each analysis uses a different NLP model to analyze the text.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash, Eq, Copy)]
#[serde(rename_all = "camelCase")]
pub enum NlpAnalysisKind {
    Clickbait,
    HateSpeech,
    Keywords,
    Language,
    Politics,
    Sarcasm,
    Sentiment,
    Spam,
}

impl NlpAnalysisKind {
    pub fn to_snake_case(&self) -> String {
        match self {
            NlpAnalysisKind::Clickbait => "clickbait",
            NlpAnalysisKind::HateSpeech => "hate_speech",
            NlpAnalysisKind::Keywords => "keywords",
            NlpAnalysisKind::Language => "language",
            NlpAnalysisKind::Politics => "politics",
            NlpAnalysisKind::Sarcasm => "sarcasm",
            NlpAnalysisKind::Sentiment => "sentiment",
            NlpAnalysisKind::Spam => "spam",
        }
        .to_string()
    }
    pub fn from_snake_case(s: &str) -> Option<Self> {
        match s {
            "clickbait" => Some(NlpAnalysisKind::Clickbait),
            "hate_speech" => Some(NlpAnalysisKind::HateSpeech),
            "keywords" => Some(NlpAnalysisKind::Keywords),
            "language" => Some(NlpAnalysisKind::Language),
            "politics" => Some(NlpAnalysisKind::Politics),
            "sarcasm" => Some(NlpAnalysisKind::Sarcasm),
            "sentiment" => Some(NlpAnalysisKind::Sentiment),
            "spam" => Some(NlpAnalysisKind::Spam),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type A = NlpAnalysisKind;
    #[test]
    fn test_to_snake_case() {
        let f = |a: A| a.to_snake_case();
        assert_eq!(f(A::Clickbait), "clickbait");
        assert_eq!(f(A::HateSpeech), "hate_speech");
        assert_eq!(f(A::Keywords), "keywords");
        assert_eq!(f(A::Language), "language");
        assert_eq!(f(A::Politics), "politics");
        assert_eq!(f(A::Sarcasm), "sarcasm");
        assert_eq!(f(A::Sentiment), "sentiment");
        assert_eq!(f(A::Spam), "spam");
    }

    #[test]
    fn test_from_snake_case() {
        let f = A::from_snake_case;
        assert_eq!(f("clickbait"), Some(A::Clickbait));
        assert_eq!(f("hate_speech"), Some(A::HateSpeech));
        assert_eq!(f("keywords"), Some(A::Keywords));
        assert_eq!(f("language"), Some(A::Language));
        assert_eq!(f("politics"), Some(A::Politics));
        assert_eq!(f("sarcasm"), Some(A::Sarcasm));
        assert_eq!(f("sentiment"), Some(A::Sentiment));
        assert_eq!(f("spam"), Some(A::Spam));
        assert_eq!(f("invalid"), None);
    }

    #[test]
    fn test_serde_camel_case() {
        let kind = A::Clickbait;
        let json = serde_json::to_string(&kind).unwrap();
        assert_eq!(json, r#""clickbait""#);
        let kind: A = serde_json::from_str(&json).unwrap();
        assert_eq!(kind, A::Clickbait);

        let kind = A::HateSpeech;
        let json = serde_json::to_string(&kind).unwrap();
        assert_eq!(json, r#""hateSpeech""#);
        let kind: A = serde_json::from_str(&json).unwrap();
        assert_eq!(kind, A::HateSpeech);
    }
}
