//! Authentication module
//!
//! Verify the user's identity

use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

/// Index route for the `auth/` module
/// Returns a welcome message
#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from /auth")
}

/// Router for the `auth/` module
/// Adds all `auth/` services to the router
pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(index);
}
