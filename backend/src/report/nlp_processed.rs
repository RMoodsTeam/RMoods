use crate::fetcher::reddit_feed_item::RedditFeedItem;
use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::nlp_analysis::NlpAnalysis;
use crate::report::reddit_data_container::RedditDataContainer;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DataExample<D> {
    data: D,
    confidence: f64,
}

type Label = String;
type Data = RedditDataContainer;

#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct ProcessedNlpAnalysis {
    kind: NlpAnalysisKind,
    generated_in: f64,
    /// Top labels for the analysis along with their percentages in descending order.
    label_percentages: HashMap<Label, f64>,
    label_counts: HashMap<Label, usize>,
    label_average_scores: HashMap<Label, f64>,
    /// Most confident data examples for each label.
    most_confidences: HashMap<Label, Vec<DataExample<Data>>>,
    /// Least confident data examples for each label.
    least_confidences: HashMap<Label, Vec<DataExample<Data>>>,
    /// Average confidence for each label.
    average_confidences: HashMap<Label, f64>,
    /// Confidence percentiles for each label.
    confidence_percentiles: HashMap<Label, Vec<f64>>,
}

fn group_by_label<L, D>(data: Vec<(, Vec<D>)>) -> HashMap<L, Vec<DataExample<D>>> {
    todo!()
}

impl ProcessedNlpAnalysis {
    fn generate_label_percentages(map: NlpAnalysis) -> HashMap<Label, f64> {
        todo!()
    }

    pub fn generate_from_analyses_map(analysis: NlpAnalysis, data: Vec<Box<dyn RedditFeedItem>>) {
        let zipped: Vec<_> = analysis.results.into_iter().zip(data.into_iter()).collect();
        let grouped = group_by_label(zipped);

        todo!()
    }
}
