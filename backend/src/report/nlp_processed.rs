use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::nlp_analysis::NlpAnalysis;
use std::collections::HashMap;

struct LabelValues<L> {
    label: L,
    percentage: f64,
}

struct DataExample<D> {
    data: D,
    confidence: f64,
}

type LabelPercentages<L> = HashMap<L, f64>;
type LabelCounts<L> = HashMap<L, usize>;
type LabelAverageScores<L> = HashMap<L, f64>;
type MostConfidences<L, D> = HashMap<L, Vec<DataExample<D>>>;
type LeastConfidences<L, D> = HashMap<L, Vec<DataExample<D>>>;
type AverageConfidences<L> = HashMap<L, f64>;
type ConfidencePercentiles<L> = HashMap<L, Vec<f64>>;

struct ProcessedNlpAnalysis<L, D> {
    kind: NlpAnalysisKind,
    generated_in: f64,
    /// Top labels for the analysis along with their percentages in descending order.
    label_percentages: LabelPercentages<L>,
    label_counts: LabelCounts<L>,
    label_average_scores: LabelAverageScores<L>,
    /// Most confident data examples for each label.
    most_confidences: MostConfidences<L, D>,
    /// Least confident data examples for each label.
    least_confidences: LeastConfidences<L, D>,
    /// Average confidence for each label.
    average_confidences: AverageConfidences<L>,
    /// Confidence percentiles for each label.
    confidence_percentiles: ConfidencePercentiles<L>,
}

fn group_by_label<L, D>(data: NlpAnalysis) -> HashMap<L, Vec<DataExample<D>>> {
    todo!()
}

impl<L, D> ProcessedNlpAnalysis<L, D> {
    fn generate_label_percentages(map: NlpAnalysis) -> LabelPercentages<L> {
        todo!()
    }

    pub fn generate_from_analyses_map(map: NlpAnalysis) {
        let label_percentages = Self::generate_label_percentages(map);
        todo!()
    }
}
