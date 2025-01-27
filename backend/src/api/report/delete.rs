use crate::app_error::AppError;
use crate::auth::user::GoogleId;
use crate::db::impls::report_query::ReportRepository;
use crate::report::report::{Report, ReportId};
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde_with::serde_derive::Serialize;

#[derive(Serialize)]
pub struct DeleteReportResponse {
    deleted: u64,
}

/// Deletes a report by ID.
///
/// Only the owner of the report can delete it.
pub async fn delete_report(
    State(state): State<AppState>,
    Path(id): Path<ReportId>,
    user_id: GoogleId,
) -> Result<Json<DeleteReportResponse>, AppError> {
    match Report::owner_id_by_id(&id, &state.db).await? {
        Some(owner_id) if owner_id == user_id => {
            let deleted = Report::delete_by_id(&id, &state.db).await?;
            Ok(Json(DeleteReportResponse { deleted }))
        }
        Some(_) => Err(AppError::unauthorized()),
        None => Err(AppError::not_found()),
    }
}
