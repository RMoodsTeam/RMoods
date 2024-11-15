use crate::AppState;
use axum::{routing::get, Router};

pub mod about;
pub mod auth;
pub mod debug;
pub mod report;
pub mod report_ack;

/// Defines routes for the /api path.
pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/about/subreddit", get(about::subreddit::subreddit_about))
        .route("/about/user", get(about::user::user_about))
        //
        .route("/debug/post-comments", get(debug::post_comments))
        .route("/debug/subreddit-posts", get(debug::subreddit_posts))
        .route("/debug/user-posts", get(debug::user_posts))
}
