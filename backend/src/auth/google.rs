use crate::auth::error::AuthError;
use crate::auth::jwt::decode_jwt;
use crate::auth::middleware::jwt_from_header_or_uri;
use crate::auth::user::{GoogleId, User};
use axum::async_trait;
use axum::extract::FromRequestParts;
use derive_getters::Getters;
use http::request::Parts;
use http::StatusCode;
use log_derive::logfn;
use reqwest::{multipart::Form, Client};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Getters)]
pub struct GoogleTokenResponse {
    access_token: String,
    expires_in: u32,
    refresh_token: String,
    scope: String,
    token_type: String,
    id_token: String,
}

#[derive(Serialize, Deserialize, Getters, Clone, Debug)]
pub struct JwtUserInfo {
    pub(crate) id: GoogleId,
}

/// Implement `FromRequestParts` for `GoogleUserInfo` to extract the user info from the request.
///
/// With this trait implemented, the user info can be extracted by any Axum HTTP handler without
/// jumping through extra hoops like obtaining an `Authorization` header and decoding the JWT.
#[async_trait]
impl<T> FromRequestParts<T> for JwtUserInfo {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &T) -> Result<Self, Self::Rejection> {
        log::warn!("Extracting GoogleUserInfo");
        let token = jwt_from_header_or_uri(&parts.headers, &parts.uri);
        let result = token
            .and_then(|token| decode_jwt(token).ok())
            .map(|claims| claims.claims.user_info);

        match result {
            Some(user_info) => {
                log::debug!("Extracted GoogleUserInfo successfully");
                Ok(user_info)
            }
            None => {
                log::error!("Failed to extract GoogleUserInfo");
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

#[logfn(err = "ERROR", fmt = "Failed to fetch access token: {:?}")]
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

/// Fetches the user info from Google using the given access token.
///
/// The access token is used to authenticate the request to the Google API.
/// The user info is then extracted from the response.
#[logfn(err = "ERROR", fmt = "Failed to fetch user info: {:?}")]
pub async fn fetch_google_user_info(
    access_token: String,
    http: &Client,
) -> Result<User, AuthError> {
    let user_info: User = http
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?
        .json()
        .await?;

    Ok(user_info)
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[test]
    fn test_google_user_info_deserialization() {
        let json = json!(
            {
                "sub": "1234567890",
                "name": "John Doe",
                "given_name": "John",
                "family_name": "Doe",
                "picture": "https://example.com/picture.jpg",
                "email": "123@example.com",
                "email_verified": true
            }
        );
        serde_json::from_value::<super::User>(json).unwrap();
    }
}
