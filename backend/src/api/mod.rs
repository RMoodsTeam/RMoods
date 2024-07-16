use std::collections::HashMap;

use axum::{routing::get, Router};

use crate::AppState;

pub mod test;

type PlainParams = HashMap<String, String>;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/test/timeout", get(test::timeout))
        .route("/test/lorem", get(test::lorem))
        .route("/test/subreddit-info", get(test::subreddit_info))
        .route("/test/subreddit-comments", get(test::subreddit_comments))
}
