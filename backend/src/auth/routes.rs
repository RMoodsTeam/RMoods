use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use super::{google::{fetch_google_access_token, GoogleUserInfo}, jwt::create_jwt};

use crate::{app_error::AppError, auth::google::fetch_google_user_info, AppState};

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    jwt: String,
    user_info: GoogleUserInfo,
}

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    code: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, AppError> {
    let auth_code = body.code;

    let auth_data = fetch_google_access_token(auth_code, &state.http).await?;

    let user_info =
        fetch_google_user_info(auth_data.access_token().to_string(), &state.http).await?;

    let jwt = create_jwt()?;

    Ok(Json(LoginResponse { jwt, user_info }))
}
