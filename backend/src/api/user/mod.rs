use crate::app_error::AppError;
use crate::api::auth::google::User;
use crate::auth::google::GoogleUserInfo;
use crate::db::db_stored::DbStored;
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
) -> Result<Json<User>, AppError> {
    User::get_by_id(&query.id, &state.db)
        .await?
        .ok_or_else(AppError::not_found)
        .map(Json)
}
