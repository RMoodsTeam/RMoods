use crate::app_error::AppError;
use crate::reddit_fetcher::reddit::ratelimit_headers::RatelimitHeaders;
use crate::AppState;
use axum::extract::State;
use axum::Json;

pub async fn rate_limits(
    State(state): State<AppState>,
) -> Result<Json<Vec<RatelimitHeaders>>, AppError> {
    let rate_limits = state.fetcher.get_latest_rate_limits().await;
    Ok(Json(rate_limits.clone()))
}
