use derive_getters::Getters;
use reqwest::{multipart::Form, Client};
use serde::{Deserialize, Serialize};

use super::error::AuthError;

#[derive(Deserialize, Debug, Getters)]
#[allow(unused)]
pub struct GoogleTokenResponse {
    access_token: String,
    expires_in: u32,
    refresh_token: String,
    scope: String,
    token_type: String,
    id_token: String,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
pub struct GoogleUserInfo {
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

pub async fn fetch_google_access_token(
    auth_code: String,
    http: &Client,
) -> Result<GoogleTokenResponse, AuthError> {
    let client_id = dotenvy::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID should be set");
    let client_secret =
        dotenvy::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET should be set");

    let form_data = Form::new()
        .text("code", auth_code)
        .text("client_id", client_id)
        .text("client_secret", client_secret)
        .text("redirect_uri", "postmessage")
        .text("grant_type", "authorization_code");

    let token: GoogleTokenResponse = http
        .post("https://oauth2.googleapis.com/token")
        .multipart(form_data)
        .send()
        .await?
        .json()
        .await?;

    Ok(token)
}

pub async fn fetch_google_user_info(
    access_token: String,
    http: &Client,
) -> Result<GoogleUserInfo, AuthError> {
    let user_info: GoogleUserInfo = http
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| AuthError::GoogleError(e))?
        .json()
        .await?;

    Ok(user_info)
}
