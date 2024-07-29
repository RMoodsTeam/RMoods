use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::error::AuthError;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    /// Expire at this timestamp
    exp: usize,
    /// Issued at this timestamp
    iat: usize,
}

/// Create a new JWT token based on the secret defined in the environment
pub fn create_jwt() -> Result<String, AuthError> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET should be set");

    let claim = {
        let now = Utc::now();
        let duration = Duration::days(30);
        let iat = now.timestamp() as usize;
        let exp = (now + duration).timestamp() as usize;
        Claims { exp, iat }
    };
    let key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());

    jsonwebtoken::encode(&Header::default(), &claim, &key).map_err(|e| AuthError::JwtError(e))
}

pub fn is_jwt_valid(token: &str) -> Result<(), jsonwebtoken::errors::Error> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET should be set");
    let key = &DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, key, &Validation::new(jsonwebtoken::Algorithm::HS256))?;
    Ok(())
}
