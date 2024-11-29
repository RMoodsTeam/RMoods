use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NlpAnalysisKind {
    Language,
    Sentiment,
    Sarcasm,
    Spam,
    Politics,
    HateSpeech,
    Clickbait,
    Keywords,
}
