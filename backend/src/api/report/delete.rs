use crate::app_error::AppError;
use crate::auth::google::JwtUserInfo;
use crate::auth::user::GoogleId;
use crate::db::impls::report_query::ReportRepository;
use crate::report::report::{Report, ReportId};
use crate::AppState;
use axum::extract::{Path, State};

/// Deletes a report by ID.
///
/// Only the owner of the report can delete it.
pub async fn delete_report(
    State(state): State<AppState>,
    Path(id): Path<ReportId>,
    jwt_user_info: JwtUserInfo,
) -> Result<(), AppError> {
    let user_id = jwt_user_info.id;
    match Report::owner_id_by_id(&id, &state.db).await? {
        Some(owner_id) if owner_id == user_id => {
            let deleted = Report::delete_by_id(&id, &state.db).await?;
            if deleted == 0 {
                return Err(AppError::not_found());
            }
            log::info!("Report {} deleted by user {}", id, user_id);
            Ok(())
        }
        Some(_) => {
            log::warn!(
                "User {} tried to delete report {} but is not the owner",
                user_id,
                id
            );
            Err(AppError::unauthorized())
        }
        None => {
            log::info!("Report {} not found", id);
            Err(AppError::not_found())
        }
    }
}
