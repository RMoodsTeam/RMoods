//! API module containing all routes
//!
//! Splits into two sub-routers:
//! 1. `reports/` - generating reports using data from X
//! 2. `actions/` - executing actions related X accounts and the user's XMoods account

use actix_web::{get, web, HttpResponse, Responder};

pub mod reports;

/// Welcome the user to our API
#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from /api")
}

/// Configure the `api/` sub-router
/// Add all `api/` services to the passed general [`web::ServiceConfig`]
pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(web::scope("/reports").configure(reports::router));
}

