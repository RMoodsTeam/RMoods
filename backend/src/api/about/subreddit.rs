use crate::api::AnyParams;
use crate::app_error::AppError;
use crate::reddit_fetcher::model::subreddit_info::SubredditAbout;
use crate::reddit_fetcher::reddit::request::SubredditAboutRequest;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use http::StatusCode;

#[utoipa::path(get, path = "/api/about/subreddit", responses(), params())]
pub async fn subreddit_about(
    State(mut state): State<AppState>,
    Query(params): Query<AnyParams>,
) -> Result<Json<SubredditAbout>, AppError> {
    let subreddit = params
        .get("r")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `subreddit` parameter"))?;
    let req = SubredditAboutRequest {
        subreddit: subreddit.to_string(),
    };
    let about = state.fetcher.fetch_about::<SubredditAbout>(req).await?;

    Ok(Json(about))
}
