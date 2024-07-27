use axum::Json;
use serde::Serialize;

use crate::app_error::AppError;

#[derive(Serialize)]
pub struct LoginResponse {
    jwt: String,
}


pub async fn login() -> Result<Json<LoginResponse>, AppError> {
    Ok(Json(LoginResponse {
        jwt: "jwt".to_string(),
    }))
}
