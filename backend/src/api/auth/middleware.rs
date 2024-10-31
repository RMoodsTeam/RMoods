use crate::api::auth::jwt::decode_jwt;
use axum::{extract::Request, middleware::Next, response::Response};
use http::{HeaderMap, StatusCode, Uri};
use log_derive::logfn;

pub fn jwt_from_header_or_uri<'a>(headers: &'a HeaderMap, uri: &'a Uri) -> Option<&'a str> {
    headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .or_else(|| uri.query().and_then(|q| q.split('=').nth(1)))
}

/// Verify the `Authorization` header and decode the JWT.
///
/// All requests to protected routes should pass through this middleware.
#[logfn(
    err = "ERROR",
    fmt = "Authorization failed: {:?}. Invalid/missing 'Authorization' header, missing fallback auth query param or invalid JWT"
)]
pub async fn authorization(request: Request, next: Next) -> Result<Response, StatusCode> {
    jwt_from_header_or_uri(request.headers(), request.uri())
        .ok_or_else(|| {
            log::error!("JWT not found in the request's Authorization header or query params.");
            StatusCode::UNAUTHORIZED
        })
        .and_then(|jwt| {
            decode_jwt(jwt).map_err(|_| {
                log::error!("Failed to decode JWT");
                StatusCode::UNAUTHORIZED
            })
        })?;

    log::info!("Authorization successful");
    Ok(next.run(request).await)
}
