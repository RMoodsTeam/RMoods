use crate::AppState;
use axum::{routing::get, Router};
use std::collections::HashMap;

pub mod test;

type AnyParams = HashMap<String, String>;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/test/timeout", get(test::timeout))
        .route("/test/lorem", get(test::lorem))
        .route("/test/subreddit-info", get(test::subreddit_info))
        .route("/test/post-comments", get(test::post_comments))
        .route("/test/user-info", get(test::user_info))
        .route("/test/subreddit-posts", get(test::subreddit_posts))
        .route("/test/user-posts", get(test::user_posts))
}
