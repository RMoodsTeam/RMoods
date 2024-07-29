use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use super::error::AuthError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    /// Expire at this timestamp
    exp: usize,
    /// Issued at this timestamp
    iat: usize,
    /// Unique user ID (subject)
    sub: String,
}

/// Create a new JWT token based on the secret defined in the environment
pub fn create_jwt(sub: String) -> Result<String, AuthError> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET should be set");

    let claim = {
        let now = Utc::now();
        let duration = Duration::days(30);
        let iat = now.timestamp() as usize;
        let exp = (now + duration).timestamp() as usize;
        Claims { exp, iat, sub }
    };
    let key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());

    jsonwebtoken::encode(&Header::default(), &claim, &key).map_err(|e| AuthError::JwtError(e))
}

pub fn is_jwt_valid(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET should be set");
    let key = &DecodingKey::from_secret(secret.as_bytes());
    let claims = decode::<Claims>(token, key, &Validation::new(jsonwebtoken::Algorithm::HS256))?;
    Ok(claims)
}
