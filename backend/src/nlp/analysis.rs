use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash, Eq)]
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
