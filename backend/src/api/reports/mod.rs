//! Report generation module
//!
//! Use data from the X API to generate reports for the user

use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

/// Index route for the `reports/` module
/// Returns a welcome message
#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from /api/reports")
}

/// Router for the `reports/` module
/// Adds all `reports/` services to the router
pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(index);
}
