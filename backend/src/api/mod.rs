use crate::AppState;
use axum::{routing::get, Router};

pub mod about;
pub mod auth;
pub mod report;

/// Defines routes for the /api path.
pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/report", get(report::generate::generate_report_handler))
        //
        .route("/about/subreddit", get(about::subreddit::subreddit_about))
        .route("/about/user", get(about::user::user_about))
}
