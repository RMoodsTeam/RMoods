use crate::api::auth::google::User;
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
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT google_id as "id: String", name, given_name, family_name, picture, email, email_verified
        FROM users
        WHERE google_id = $1
        "#,
        query.id
    )
        .fetch_one(state.db.raw_db())
        .await?;

    Ok(Json(user))
}
