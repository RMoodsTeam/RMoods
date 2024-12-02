use crate::api::auth::google::GoogleUserInfo;
use crate::app_error::AppError;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
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
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(user))
}
