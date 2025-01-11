use crate::nlp::analysis_kind::NlpAnalysisKind;
use std::collections::HashMap;

struct LabelValues<L> {
    label: L,
    percentage: f64,
}

struct DataExample<D> {
    data: D,
    confidence: f64,
}

struct ProcessedNlpAnalysis<D, L> {
    kind: NlpAnalysisKind,
    generated_in: f64,
    /// Top labels for the analysis along with their percentages in descending order.
    label_percentages: HashMap<L, f64>,
    /// Examples of data with the most and least confidence for each label.
    most_confidences: HashMap<L, Vec<DataExample<D>>>,
    /// Examples of data with the least confidence for each label.
    least_confidences: HashMap<L, Vec<DataExample<D>>>,
    /// Average confidence for each label.
    average_confidences: HashMap<L, f64>,
    /// Confidence percentiles for each label.
    confidence_percentiles: HashMap<L, Vec<f64>>,
}
