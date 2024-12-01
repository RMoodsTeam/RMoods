use serde::{Deserialize, Serialize};

/// The kind of analysis to perform.
/// This is used to determine the endpoint to hit on the NLP service.
/// The NLP service will return a response with the same kind of analysis.
///
/// Each analysis uses a different NLP model to analyze the text.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[serde(rename_all = "snake_case")]
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
