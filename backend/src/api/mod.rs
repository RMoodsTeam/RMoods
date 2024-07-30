use crate::AppState;
use axum::{routing::get, Router};
use std::collections::HashMap;

pub mod debug;
pub mod report;

type AnyParams = HashMap<String, String>;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/debug/timeout", get(debug::timeout))
        .route("/debug/lorem", get(debug::lorem))
        .route("/debug/subreddit-info", get(debug::subreddit_info))
        .route("/debug/post-comments", get(debug::post_comments))
        .route("/debug/user-info", get(debug::user_info))
        .route("/debug/subreddit-posts", get(debug::subreddit_posts))
        .route("/debug/user-posts", get(debug::user_posts))
}
