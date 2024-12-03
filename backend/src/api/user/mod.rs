use crate::app_error::AppError;
use crate::auth::google::GoogleUserInfo;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetUserQuery {
    id: String,
}

pub async fn get_user(
    State(state): State<AppState>,
    Query(query): Query<GetUserQuery>,
) -> Result<Json<GoogleUserInfo>, AppError> {
    let user = sqlx::query_as!(
        GoogleUserInfo,
        "SELECT * FROM users WHERE id = $1",
        query.id
    )
    .fetch_optional(&state.pool)
    .await?;

    match user {
        None => Err(AppError::new(StatusCode::NOT_FOUND, "User not found")),
        Some(user) => Ok(Json(user)),
    }
}
