use crate::api::AnyParams;
use crate::app_error::AppError;
use crate::reddit_fetcher::model::user_info::UserAbout;
use crate::reddit_fetcher::reddit::request::UserAboutRequest;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use http::StatusCode;

#[utoipa::path(get, path = "/api/about/user", responses(), params())]
pub async fn user_about(
    State(mut state): State<AppState>,
    Query(params): Query<AnyParams>,
) -> Result<Json<UserAbout>, AppError> {
    let user = params
        .get("u")
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Missing `u` parameter"))?;
    let req = UserAboutRequest {
        username: user.to_string(),
    };
    let about = state.fetcher.fetch_about::<UserAbout>(req).await?;
    Ok(Json(about))
}
