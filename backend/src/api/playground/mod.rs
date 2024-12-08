use crate::app_error::AppError;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpAnalysis;
use crate::AppState;
use axum::extract::{Query, State};
use axum::{debug_handler, Json};
use log_derive::logfn;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlaygroundQuery {
    analysis: NlpAnalysisKind,
}

#[derive(Deserialize)]
pub struct PlaygroundBody {
    text: Vec<String>,
}

#[debug_handler]
#[logfn(err = "ERROR", fmt = "Failed to analyze text in playground: {:?}")]
pub async fn playground(
    State(state): State<AppState>,
    query: Query<PlaygroundQuery>,
    input: Json<PlaygroundBody>,
) -> Result<Json<NlpAnalysis>, AppError> {
    let analysis = state
        .nlp_client
        .analyze(query.analysis, &input.text)
        .await?;
    Ok(Json(analysis))
}
