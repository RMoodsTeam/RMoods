use crate::AppState;
use axum::{routing::get, Router};
use std::collections::HashMap;

pub mod debug;
pub mod report;

/// A hashmap of any type of query parameters.
type AnyParams = HashMap<String, String>;

/// Defines routes for the /api path.
pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/debug/timeout", get(debug::timeout))
        .route("/debug/lorem", get(debug::lorem))
        .route("/debug/subreddit-info", get(debug::subreddit_info))
        .route("/debug/post-comments", get(debug::post_comments))
        .route("/debug/user-info", get(debug::user_info))
        .route("/debug/subreddit-posts", get(debug::subreddit_posts))
        .route("/debug/user-posts", get(debug::user_posts))
        .route("/report/sentiment", get(report::sentiment))
        .route("/report/language", get(report::language))
        .route("/report/sarcasm", get(report::sarcasm))
        .route("/report/keywords", get(report::keywords))
        .route("/report/spam", get(report::spam))
        .route("/report/politics", get(report::politics))
        .route("/report/hate-speech", get(report::hate_speech))
        .route("/report/clickbait", get(report::clickbait))
        .route("/report/troll", get(report::troll))
}
