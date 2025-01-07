use crate::app_error::AppError;
use crate::auth::google::JwtUserInfo;
use crate::db::impls::report_query::{ReportQuery, ReportQueryResult, ReportRepository};
use crate::report::report::Report;
use crate::AppState;
use axum::extract::State;
use axum::{debug_handler, Json};
use axum_extra::extract::{Query, QueryRejection};
use log_derive::logfn;
use serde::Serialize;

#[debug_handler]
#[logfn(err = "ERROR", fmt = "Failed to get reports by query: {0:?}")]
pub async fn get_reports(
    State(state): State<AppState>,
    query: Result<Query<ReportQuery>, QueryRejection>,
    user_info: JwtUserInfo,
) -> Result<Json<ReportQueryResult>, AppError> {
    match query {
        Ok(Query(query)) => {
            log::debug!("Getting reports by query: {:?}", query);
            let report_query_result = Report::get_by_query(query, user_info.id, &state.db).await?;
            log::debug!("Returning {:?} reports", report_query_result.reports.len());
            Ok(Json(report_query_result))
        }
        Err(rejection) => {
            log::error!("Failed to parse report query: {:?}", rejection);
            Err(AppError::internal_server_error())
        }
    }
}
