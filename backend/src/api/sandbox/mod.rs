use crate::app_error::AppError;
use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::nlp_analysis::NlpAnalysis;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use log_derive::logfn;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlaygroundQuery {
    analysis: NlpAnalysisKind,
}

#[derive(Deserialize)]
pub struct PlaygroundBody {
    text: String,
}

#[logfn(err = "ERROR", fmt = "Failed to analyze text in playground: {:?}")]
pub async fn playground(
    State(state): State<AppState>,
    query: Query<PlaygroundQuery>,
    input: Json<PlaygroundBody>,
) -> Result<Json<NlpAnalysis>, AppError> {
    let analysis = state
        .nlp_client
        .analyze(query.analysis, &vec![input.text.clone()])
        .await?;
    Ok(Json(analysis))
}
