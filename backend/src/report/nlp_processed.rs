use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::nlp_analysis::NlpAnalysis;
use crate::nlp::nlp_response::NlpResponse;
use serde::Serialize;
use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DataExample<D> {
    data: D,
    confidence: f64,
}

type Label = String;

#[derive(Debug, Clone, PartialEq, Serialize)]
struct GroupedData<T>(HashMap<Label, Vec<DataExample<T>>>)
where
    T: Clone;

impl<T> Deref for GroupedData<T>
where
    T: Clone,
{
    type Target = HashMap<Label, Vec<DataExample<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct ProcessedNlpAnalysis<T> {
    kind: NlpAnalysisKind,
    generated_in: f64,
    /// Top labels for the analysis along with their percentages in descending order.
    label_percentages: HashMap<Label, f64>,
    label_counts: HashMap<Label, usize>,
    /// Most confident data examples for each label.
    most_confidences: HashMap<Label, Vec<DataExample<T>>>,
    /// Least confident data examples for each label.
    least_confidences: HashMap<Label, Vec<DataExample<T>>>,
    /// Average confidence for each label.
    average_confidences: HashMap<Label, f64>,
    /// Confidence percentiles for each label.
    confidence_percentiles: HashMap<Label, Vec<f64>>,
}

fn group_by_label<T>(data: Vec<(NlpResponse, T)>) -> GroupedData<T>
where
    T: Clone,
{
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
    GroupedData(grouped)
}

impl<T: Clone> ProcessedNlpAnalysis<T> {
    fn generate_label_counts(map: GroupedData<T>) -> HashMap<Label, usize> {
        let mut label_counts = HashMap::new();
        for (label, data) in map.0 {
            label_counts.insert(label, data.len());
        }
        label_counts
    }

    fn generate_label_percentages(map: GroupedData<T>) -> HashMap<Label, f64> {
        let total: usize = map.0.values().map(|v| v.len()).sum();
        map.0
            .into_iter()
            .map(|(label, data)| {
                let percentage = data.len() as f64 / total as f64;
                let rounded_percentage = (percentage * 100.0).round() / 100.0;
                (label.clone(), rounded_percentage)
            })
            .collect()
    }

    fn generate_most_confidences(
        grouped_data: GroupedData<T>,
    ) -> HashMap<Label, Vec<DataExample<T>>> {
        // top 10 confidences
        grouped_data
            .0
            .into_iter()
            .map(|(label, mut data)| {
                data.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
                (label.clone(), data.into_iter().take(10).collect())
            })
            .collect()
    }

    fn generate_least_confidences(
        grouped_data: GroupedData<T>,
    ) -> HashMap<Label, Vec<DataExample<T>>> {
        // bottom 10 confidences
        grouped_data
            .0
            .into_iter()
            .map(|(label, mut data)| {
                data.sort_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());
                (label.clone(), data.into_iter().take(10).collect())
            })
            .collect()
    }

    fn generate_average_confidences(grouped_data: GroupedData<T>) -> HashMap<Label, f64> {
        grouped_data
            .0
            .into_iter()
            .map(|(label, data)| {
                let total: f64 = data.iter().map(|d| d.confidence).sum();
                let average = total / data.len() as f64;
                let rounded_avg = (average * 100.0).round() / 100.0;
                (label.clone(), rounded_avg)
            })
            .collect()
    }

    fn generate_confidence_percentiles(grouped_data: GroupedData<T>) -> HashMap<Label, Vec<f64>> {
        grouped_data
            .0
            .into_iter()
            .map(|(label, mut data)| {
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

    pub fn generate_from_analyses_map(analysis: &NlpAnalysis, data: Vec<T>) -> Self {
        let zipped: Vec<_> = analysis
            .clone()
            .results
            .into_iter()
            .zip(data.into_iter())
            .collect();
        let grouped = group_by_label(zipped);

        let label_counts = Self::generate_label_counts(grouped.clone());
        let label_percentages = Self::generate_label_percentages(grouped.clone());
        let most_confidences = Self::generate_most_confidences(grouped.clone());
        let least_confidences = Self::generate_least_confidences(grouped.clone());
        let average_confidences = Self::generate_average_confidences(grouped.clone());
        let confidence_percentiles = Self::generate_confidence_percentiles(grouped);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nlp::analysis_kind::NlpAnalysisKind;
    use crate::nlp::nlp_analysis::NlpAnalysis;
    use crate::nlp::nlp_response::NlpResponse;
    use serde_json::{json, Value};

    fn get_test_analysis() -> NlpAnalysis {
        NlpAnalysis {
            kind: NlpAnalysisKind::Language,
            generated_in: 0.0,
            results: vec![
                NlpResponse {
                    label: "label1".to_string(),
                    confidence: 0.5,
                },
                NlpResponse {
                    label: "label1".to_string(),
                    confidence: 0.9,
                },
                NlpResponse {
                    label: "label2".to_string(),
                    confidence: 0.6,
                },
                NlpResponse {
                    label: "label1".to_string(),
                    confidence: 0.7,
                },
                NlpResponse {
                    label: "label2".to_string(),
                    confidence: 0.8,
                },
            ],
        }
    }

    fn get_test_data() -> Vec<Value> {
        vec![
            json!("data1"),
            json!("data2"),
            json!("data3"),
            json!("data4"),
            json!("data5"),
        ]
    }

    #[test]
    fn test_group_by_label() {
        let analysis = get_test_analysis();
        let data = get_test_data();
        let zipped: Vec<_> = analysis.results.into_iter().zip(data.into_iter()).collect();
        let grouped = group_by_label(zipped);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped["label1"].len(), 3);
        assert_eq!(grouped["label2"].len(), 2);
    }

    #[test]
    fn test_generate_label_counts() {
        let analysis = get_test_analysis();
        let grouped = group_by_label(
            analysis
                .results
                .into_iter()
                .zip(get_test_data().into_iter())
                .collect(),
        );
        let counts = ProcessedNlpAnalysis::generate_label_counts(grouped);
        assert_eq!(counts.len(), 2);
        assert_eq!(counts["label1"], 3);
        assert_eq!(counts["label2"], 2);
    }

    #[test]
    fn test_generate_label_percentages() {
        let analysis = get_test_analysis();
        let grouped = group_by_label(
            analysis
                .results
                .into_iter()
                .zip(get_test_data().into_iter())
                .collect(),
        );
        let percentages = ProcessedNlpAnalysis::generate_label_percentages(grouped);
        assert_eq!(percentages.len(), 2);
        assert_eq!(percentages["label1"], 0.6);
        assert_eq!(percentages["label2"], 0.4);
    }

    #[test]
    fn test_generate_most_confidences() {
        let analysis = get_test_analysis();
        let data = get_test_data();
        let zipped: Vec<_> = analysis.results.into_iter().zip(data.into_iter()).collect();
        let grouped = group_by_label(zipped);
        let most_confidences = ProcessedNlpAnalysis::generate_most_confidences(grouped);
        assert_eq!(most_confidences.len(), 2);
        assert_eq!(most_confidences["label1"].len(), 3);
        assert_eq!(most_confidences["label2"].len(), 2);
        assert_eq!(most_confidences["label1"][0].data, json!("data2"));
        assert_eq!(most_confidences["label2"][0].data, json!("data5"));
    }

    #[test]
    fn test_generate_least_confidences() {
        let analysis = get_test_analysis();
        let data = get_test_data();
        let zipped: Vec<_> = analysis.results.into_iter().zip(data.into_iter()).collect();
        let grouped = group_by_label(zipped);
        let least_confidences = ProcessedNlpAnalysis::generate_least_confidences(grouped);
        assert_eq!(least_confidences.len(), 2);
        assert_eq!(least_confidences["label1"].len(), 3);
        assert_eq!(least_confidences["label2"].len(), 2);
        assert_eq!(least_confidences["label1"][0].data, json!("data1"));
        assert_eq!(least_confidences["label2"][0].data, json!("data3"));
    }

    #[test]
    fn test_generate_average_confidences() {
        let analysis = get_test_analysis();
        let data = get_test_data();
        let zipped: Vec<_> = analysis.results.into_iter().zip(data.into_iter()).collect();
        let grouped = group_by_label(zipped);
        let average_confidences = ProcessedNlpAnalysis::generate_average_confidences(grouped);
        assert_eq!(average_confidences.len(), 2);
        assert_eq!(average_confidences["label1"], 0.7);
        assert_eq!(average_confidences["label2"], 0.7);
    }

    #[test]
    fn test_generate_confidence_percentiles() {
        let analysis = get_test_analysis();
        let data = get_test_data();
        let zipped: Vec<_> = analysis.results.into_iter().zip(data.into_iter()).collect();
        let grouped = group_by_label(zipped);
        let confidence_percentiles = ProcessedNlpAnalysis::generate_confidence_percentiles(grouped);
        assert_eq!(confidence_percentiles.len(), 2);
        assert_eq!(confidence_percentiles["label1"].len(), 3);
        assert_eq!(confidence_percentiles["label2"].len(), 3);
        assert_eq!(confidence_percentiles["label1"][0], 0.5);
        assert_eq!(confidence_percentiles["label1"][1], 0.7);
        assert_eq!(confidence_percentiles["label1"][2], 0.9);
        assert_eq!(confidence_percentiles["label2"][0], 0.6);
        assert_eq!(confidence_percentiles["label2"][1], 0.8);
        assert_eq!(confidence_percentiles["label2"][2], 0.8);
    }

    #[test]
    fn test_generate_from_analyses_map() {
        let analysis = get_test_analysis();
        let data = get_test_data();
        let processed = ProcessedNlpAnalysis::generate_from_analyses_map(&analysis, data);
        assert_eq!(processed.kind, NlpAnalysisKind::Language);
        assert_eq!(processed.generated_in, 0.0);
        assert_eq!(processed.label_counts.len(), 2);
        assert_eq!(processed.label_percentages.len(), 2);
        assert_eq!(processed.most_confidences.len(), 2);
        assert_eq!(processed.least_confidences.len(), 2);
        assert_eq!(processed.average_confidences.len(), 2);
        assert_eq!(processed.confidence_percentiles.len(), 2);
    }
}
