use axum::{extract::Request, middleware::Next, response::Response};
use http::StatusCode;
use log::info;

use crate::auth::jwt::is_jwt_valid;

pub async fn authorization(request: Request, next: Next) -> Result<Response, StatusCode> {
    let user_jwt = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    info!("Authorization header: {}", user_jwt);

    let token_data = is_jwt_valid(user_jwt).map_err(|_| StatusCode::UNAUTHORIZED)?;

    dbg!(token_data);

    info!("JWT is valid");

    Ok(next.run(request).await)
}
