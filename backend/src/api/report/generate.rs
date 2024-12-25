use crate::api::report::report_ack::ReportAck;
use crate::app_error::AppError;
use crate::auth::google::JwtUserInfo;
use crate::auth::user::GoogleId;
use crate::db::db_stored::DbStored;
use crate::report::report::{Report, ReportId};
use crate::report::report_request::ReportRequest;
use crate::report::report_status::ReportStatus;
use crate::validation::validated::Validated;
use crate::websocket::SystemMessage;
use crate::AppState;
use axum::extract::State;

/// Creates a new report and generates analyses for it.
///
/// First, a new report is created with the provided request and user ID.
/// It gets written to the database as "in progress".
/// Then, the report generates analyses based on the request and updates itself in the database.
async fn generate_report(
    report_request: ReportRequest,
    user_id: GoogleId,
    state: &mut AppState,
) -> Result<Report, (Report, AppError)> {
    let mut report = Report::empty_in_progress(report_request.clone(), user_id.clone());
    report
        .save(&state.db)
        .await
        .map_err(|e| (report.clone(), AppError::from(e)))?;

    let analyses = report
        .generate_analyses(report_request.clone(), state)
        .await
        .map_err(|e| (report.clone(), AppError::from(e)))?;

    report.fill(analyses);

    match report.update(&state.db).await {
        Ok(_) => log::debug!("Report updated with analyses and saved as successful"),
        Err(e) => {
            log::error!("Failed to update report with analyses: {:?}", e);
            report
                .delete(&state.db)
                .await
                .map_err(|e| (report.clone(), AppError::from(e)))?;
        }
    }

    Ok(report)
}

pub async fn generate_report_handler(
    State(mut state): State<AppState>,
    user_info: JwtUserInfo,
    report_request: ReportRequest,
) -> Result<ReportAck, AppError> {
    log::debug!("Validating feed request: {:?}", report_request);
    report_request.validate()?;
    log::debug!("Feed request is valid");
    log::debug!("Generating report for user: {}", user_info.id);

    tokio::spawn(async move {
        // Generate the report in a separate task
        match generate_report(report_request.clone(), user_info.id.clone(), &mut state).await {
            Ok(report) => {
                // If the report was generated successfully, send a message to the system, it's already updated in the database
                log::debug!("Report generated successfully");
                state
                    .system_tx
                    .send(SystemMessage::ReportDone((report.id, user_info.id)))
                    .await
                    .expect("Failed to send message");
            }
            Err((mut report, error)) => {
                // If an error occurred, update the report with the error status and send a message to the system
                report.status = ReportStatus::Error(error.message().to_string());
                match report.update(&state.db).await {
                    Ok(_) => log::debug!("Report updated with error status: {:?}", report.status),
                    Err(e) => {
                        // If an error occurred while updating the report, send a message to the system and delete the report.
                        // Ignore the result of the deletion, as the report is already in an error state
                        log::error!("Failed to update report with error status: {:?}", e);
                        state
                            .system_tx
                            .send(SystemMessage::ReportError((
                                AppError::from(e),
                                user_info.id.clone(),
                            )))
                            .await
                            .expect("Failed to send message");
                        report
                            .delete(&state.db)
                            .await
                            .expect("Failed to delete report");
                    }
                }
                state
                    .system_tx
                    .send(SystemMessage::ReportDone((report.id, user_info.id)))
                    .await
                    .expect("Failed to send message");
                log::error!("Failed to generate report: {:?}", error);
            }
        }
    });

    Ok(ReportAck::new())
}
