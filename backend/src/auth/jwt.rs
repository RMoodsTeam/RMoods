use chrono::{Duration, Utc};
use jsonwebtoken::Header;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    /// Expire at this timestamp
    exp: usize,
    /// Issued at this timestamp
    iat: usize,
}

/// Create a new JWT token based on the secret defined in the environment
pub fn create_jwt() -> Result<String, StatusCode> {
    let secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET should be set");

    let claim = {
        let now = Utc::now();
        let duration = Duration::seconds(15);
        let iat = now.timestamp() as usize;
        let exp = (now + duration).timestamp() as usize;
        Claims { exp, iat }
    };
    let key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());

    jsonwebtoken::encode(&Header::default(), &claim, &key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_jwt_valid() -> Result<bool, StatusCode> {
    todo!()
}
