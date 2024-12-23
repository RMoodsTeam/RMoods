use crate::app_error::AppError;
use crate::auth::google::JwtUserInfo;
use crate::db::impls::report_repository::{ReportQuery, ReportRepository};
use crate::report::report::Report;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::Query;
use log_derive::logfn;

#[logfn(err = "ERROR", fmt = "Failed to get reports by query: {0:?}")]
pub async fn get_reports(
    State(state): State<AppState>,
    Query(query): Query<ReportQuery>,
    user_info: JwtUserInfo,
) -> Result<Json<Vec<Report>>, AppError> {
    log::debug!("Getting reports by query: {:?}", query);
    let reports = Report::get_by_query(query, user_info.id, &state.db).await?;
    log::debug!("Returning {:?} reports", reports.len());
    Ok(Json(reports))
}
