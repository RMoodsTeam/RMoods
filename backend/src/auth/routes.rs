use axum::Json;
use serde::{Deserialize, Serialize};

use crate::app_error::AppError;

#[derive(Serialize)]
pub struct LoginResponse {
    jwt: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    code: String,
}

pub async fn login(Json(body): Json<LoginPayload>) -> Result<Json<LoginResponse>, AppError> {
    dbg!(body.code);
    Ok(Json(LoginResponse {
        jwt: "jwt".to_string(),
    }))
}
