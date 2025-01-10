use crate::app_error::AppError;
use crate::fetcher::model::subreddit_info::SubredditAbout;
use crate::fetcher::reddit::request::SubredditAboutRequest;
use crate::AppState;
use axum::extract::State;
use axum::Json;

pub async fn subreddit_about(
    State(mut state): State<AppState>,
    Json(request): Json<SubredditAboutRequest>,
) -> Result<Json<SubredditAbout>, AppError> {
    let about = state.fetcher.fetch_about::<SubredditAbout>(request).await?;
    Ok(Json(about))
}
