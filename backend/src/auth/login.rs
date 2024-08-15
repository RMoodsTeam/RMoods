use axum::{extract::State, Json};
use log_derive::logfn;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{google::fetch_google_access_token, jwt::create_jwt};

use crate::{app_error::AppError, auth::google::fetch_google_user_info, AppState};

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

    let jwt = create_jwt(user_info);

    Ok(Json(LoginResponse { jwt }))
}
