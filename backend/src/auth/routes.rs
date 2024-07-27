use axum::{extract::State, Json};
use reqwest::{multipart::Form, Url};
use serde::{Deserialize, Serialize};

use crate::{app_error::AppError, AppState};

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    jwt: String,
    user_info: GoogleUserInfo,
}

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    code: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct GoogleTokenResponse {
    access_token: String,
    expires_in: u32,
    refresh_token: String,
    scope: String,
    token_type: String,
    id_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct GoogleUserInfo {
    /// Unique user ID
    sub: String,
    name: String,
    given_name: String,
    family_name: String,
    /// URL to the user's picture
    picture: String,
    email: String,
    email_verified: bool,
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, AppError> {
    let auth_code = body.code;

    let client_id = dotenvy::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID should be set");
    let client_secret =
        dotenvy::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET should be set");

    let form_data = Form::new()
        .text("code", auth_code)
        .text("client_id", client_id)
        .text("client_secret", client_secret)
        .text("redirect_uri", "postmessage")
        .text("grant_type", "authorization_code");

    let token_response: GoogleTokenResponse = state
        .http
        .post("https://oauth2.googleapis.com/token")
        .multipart(form_data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let user_info: GoogleUserInfo = state
        .http
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token_response.access_token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(Json(LoginResponse {
        jwt: "jwt".to_string(),
        user_info,
    }))
}
