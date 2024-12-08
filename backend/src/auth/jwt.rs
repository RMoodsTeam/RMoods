use crate::auth::error::AuthError;
use crate::auth::google::{JwtUserInfo, User};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Header, TokenData, Validation};
use log_derive::logfn;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    /// Expire at this timestamp
    pub exp: usize,
    /// Issued at this timestamp
    pub iat: usize,
    pub user_info: JwtUserInfo,
}

/// Create a new JWT based on the secret defined in the environment.
pub fn create_jwt(user_info: User) -> String {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET should be set");

    let claim = {
        let now = Utc::now();
        let duration = Duration::days(30);
        let iat = now.timestamp() as usize;
        let exp = (now + duration).timestamp() as usize;
        Claims {
            exp,
            iat,
            user_info: JwtUserInfo { id: user_info.id },
        }
    };
    let key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());

    jsonwebtoken::encode(&Header::default(), &claim, &key)
        .expect("Failed to encode JWT, unrecoverable")
}

/// Decode given JWT, verifying it at the same time. Return the decoded token data.
#[logfn(err = "ERROR", fmt = "Failed to decode JWT: {:?}")]
pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, AuthError> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET should be set");
    let key = &DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(token, key, &Validation::new(jsonwebtoken::Algorithm::HS256))
        .map_err(AuthError::JwtError)
}
