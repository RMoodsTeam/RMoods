use crate::app_error::AppError;
use crate::db::impls::report_repository::ReportQuery;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::Query;
use log_derive::logfn;

#[logfn(err = "ERROR", fmt = "Failed to get reports by query: {0:?}")]
pub async fn get_reports(
    State(state): State<AppState>,
    Query(query): Query<ReportQuery>,
) -> Result<impl IntoResponse, AppError> {
    dbg!(query);

    Ok(())
}
