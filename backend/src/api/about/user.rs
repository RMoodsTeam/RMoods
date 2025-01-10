use crate::app_error::AppError;
use crate::fetcher::model::user_info::UserAbout;
use crate::fetcher::reddit::request::UserAboutRequest;
use crate::AppState;
use axum::extract::State;
use axum::Json;

pub async fn user_about(
    State(mut state): State<AppState>,
    Json(request): Json<UserAboutRequest>,
) -> Result<Json<UserAbout>, AppError> {
    let about = state.fetcher.fetch_about::<UserAbout>(request).await?;
    Ok(Json(about))
}
