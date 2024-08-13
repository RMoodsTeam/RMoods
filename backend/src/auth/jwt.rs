use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Header, TokenData, Validation};
use log_derive::logfn;
use serde::{Deserialize, Serialize};

use super::{error::AuthError, google::GoogleUserInfo};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    /// Expire at this timestamp
    exp: usize,
    /// Issued at this timestamp
    iat: usize,
    pub user_info: GoogleUserInfo,
}

/// Create a new JWT token based on the secret defined in the environment
#[logfn(err = "ERROR", fmt = "Failed to create JWT: {:?}")]
pub fn create_jwt(user_info: GoogleUserInfo) -> Result<String, AuthError> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET should be set");

    let claim = {
        let now = Utc::now();
        let duration = Duration::days(30);
        let iat = now.timestamp() as usize;
        let exp = (now + duration).timestamp() as usize;
        Claims {
            exp,
            iat,
            user_info,
        }
    };
    let key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());

    jsonwebtoken::encode(&Header::default(), &claim, &key).map_err(AuthError::JwtError)
}

#[logfn(err = "ERROR", fmt = "Failed to decode JWT: {:?}")]
pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, AuthError> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET should be set");
    let key = &DecodingKey::from_secret(secret.as_bytes());
    
    decode::<Claims>(token, key, &Validation::new(jsonwebtoken::Algorithm::HS256))
        .map_err(AuthError::JwtError)
}
