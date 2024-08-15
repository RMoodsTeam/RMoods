use axum::{extract::Request, middleware::Next, response::Response};
use http::StatusCode;
use log::info;
use log_derive::logfn;

use crate::auth::jwt::decode_jwt;

/// Verify the `Authorization` header and decode the JWT.
///
/// All requests to protected routes should pass through this middleware.
#[logfn(err = "ERROR", fmt = "Authorization failed: {:?}")]
pub async fn authorization(request: Request, next: Next) -> Result<Response, StatusCode> {
    let user_jwt = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    info!("Authorization header: {}", user_jwt);

    decode_jwt(user_jwt).map_err(|_| StatusCode::UNAUTHORIZED)?;

    info!("JWT is valid");

    Ok(next.run(request).await)
}
