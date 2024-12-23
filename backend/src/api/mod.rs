use crate::AppState;
use axum::routing::post;
use axum::{routing::get, Router};

pub mod about;
pub mod auth;
pub mod report;
mod sandbox;
mod system;
mod user;

/// Defines routes for the /api path.
pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/report", post(report::generate::generate_report_handler))
        .route("/report", get(report::get::get_reports))
        .route("/about/subreddit", get(about::subreddit::subreddit_about))
        .route("/about/user", get(about::user::user_about))
        .route("/user", get(user::get_user))
        .route("/system/rate-limits", get(system::rate_limits))
        .route("/sandbox", post(sandbox::playground))
}
