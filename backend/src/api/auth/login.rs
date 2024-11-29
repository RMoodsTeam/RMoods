use axum::{extract::State, Json};
use log_derive::logfn;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use utoipa::ToSchema;

use super::{google::fetch_google_access_token, jwt::create_jwt};

use crate::api::auth::google::fetch_google_user_info;
use crate::{app_error::AppError, AppState};

#[derive(Serialize, Debug, ToSchema)]
pub struct LoginResponse {
    jwt: String,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct LoginPayload {
    code: String,
}

#[utoipa::path(post, path = "/auth/login", responses(), params())]
#[logfn(err = "ERROR", fmt = "'login' failed: {:?}")]
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, AppError> {
    let auth_data = fetch_google_access_token(body.code, &state.http).await?;

    let user_info =
        fetch_google_user_info(auth_data.access_token().to_string(), &state.http).await?;

    sqlx::query!(
        r#"INSERT INTO users (
    id, name, given_name, family_name, picture, email, email_verified)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    ON CONFLICT (id) DO UPDATE
    SET name = $2, given_name = $3, family_name = $4, picture = $5, email = $6, email_verified = $7
    "#,
        user_info.id,
        user_info.name,
        user_info.given_name,
        user_info.family_name,
        user_info.picture,
        user_info.email,
        user_info.email_verified
    )
    .execute(&state.pool)
    .await?;

    log::debug!("User logged in: {:?}", user_info);

    let jwt = create_jwt(user_info);

    Ok(Json(LoginResponse { jwt }))
}
