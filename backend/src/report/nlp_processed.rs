use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::nlp_analysis::NlpAnalysis;
use crate::nlp::nlp_response::NlpResponse;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DataExample<D> {
    data: D,
    confidence: f64,
}

type Label = String;
type DataType = Value;
type GroupedData = HashMap<Label, Vec<DataExample<DataType>>>;

#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct ProcessedNlpAnalysis {
    kind: NlpAnalysisKind,
    generated_in: f64,
    /// Top labels for the analysis along with their percentages in descending order.
    label_percentages: HashMap<Label, f64>,
    label_counts: HashMap<Label, usize>,
    /// Most confident data examples for each label.
    most_confidences: HashMap<Label, Vec<DataExample<DataType>>>,
    /// Least confident data examples for each label.
    least_confidences: HashMap<Label, Vec<DataExample<DataType>>>,
    /// Average confidence for each label.
    average_confidences: HashMap<Label, f64>,
    /// Confidence percentiles for each label.
    confidence_percentiles: HashMap<Label, Vec<f64>>,
}

fn group_by_label(data: Vec<(NlpResponse, DataType)>) -> GroupedData {
    let mut grouped = HashMap::new();
    for (response, data) in data {
        let label = response.label;
        let confidence = response.confidence;
        let data_example = DataExample { data, confidence };
        grouped
            .entry(label)
            .or_insert_with(Vec::new)
            .push(data_example);
    }
    GroupedData::from(grouped)
}

impl ProcessedNlpAnalysis {
    fn generate_label_counts(map: NlpAnalysis) -> HashMap<Label, usize> {
        let mut label_counts = HashMap::new();
        for response in map.results {
            let label = response.label;
            *label_counts.entry(label).or_insert(0) += 1;
        }
        label_counts
    }

    fn generate_label_percentages(map: NlpAnalysis) -> HashMap<Label, f64> {
        let total = map.results.len() as f64;
        let mut label_counts = HashMap::new();
        for response in map.results {
            let label = response.label;
            *label_counts.entry(label).or_insert(0) += 1;
        }
        label_counts
            .into_iter()
            .map(|(label, count)| (label, count as f64 / total))
            .collect()
    }

    fn generate_most_confidences(
        grouped_data: &GroupedData,
    ) -> HashMap<Label, Vec<DataExample<DataType>>> {
        // top 10 confidences
        grouped_data
            .iter()
            .map(|(label, data)| {
                let mut data = data.clone();
                data.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
                (label.clone(), data.into_iter().take(10).collect())
            })
            .collect()
    }

    fn generate_least_confidences(
        grouped_data: &GroupedData,
    ) -> HashMap<Label, Vec<DataExample<DataType>>> {
        // bottom 10 confidences
        grouped_data
            .iter()
            .map(|(label, data)| {
                let mut data = data.clone();
                data.sort_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());
                (label.clone(), data.into_iter().take(10).collect())
            })
            .collect()
    }

    fn generate_average_confidences(grouped_data: &GroupedData) -> HashMap<Label, f64> {
        grouped_data
            .iter()
            .map(|(label, data)| {
                let total: f64 = data.iter().map(|d| d.confidence).sum();
                let average = total / data.len() as f64;
                (label.clone(), average)
            })
            .collect()
    }

    fn generate_confidence_percentiles(grouped_data: &GroupedData) -> HashMap<Label, Vec<f64>> {
        grouped_data
            .iter()
            .map(|(label, data)| {
                let mut data = data.clone();
                data.sort_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());
                let percentiles = vec![0.25, 0.5, 0.75]
                    .into_iter()
                    .map(|p| {
                        let index = (data.len() as f64 * p) as usize;
                        data[index].confidence
                    })
                    .collect();
                (label.clone(), percentiles)
            })
            .collect()
    }

    pub fn generate_from_analyses_map(analysis: &NlpAnalysis, data: Vec<Value>) -> Self {
        let zipped: Vec<_> = analysis
            .clone()
            .results
            .into_iter()
            .zip(data.into_iter())
            .collect();
        let grouped = group_by_label(zipped);

        let label_counts = Self::generate_label_counts(analysis.clone());
        let label_percentages = Self::generate_label_percentages(analysis.clone());
        let most_confidences = Self::generate_most_confidences(&grouped);
        let least_confidences = Self::generate_least_confidences(&grouped);
        let average_confidences = Self::generate_average_confidences(&grouped);
        let confidence_percentiles = Self::generate_confidence_percentiles(&grouped);

        Self {
            kind: analysis.kind,
            generated_in: analysis.generated_in,
            label_percentages,
            label_counts,
            most_confidences,
            least_confidences,
            average_confidences,
            confidence_percentiles,
        }
    }
}
