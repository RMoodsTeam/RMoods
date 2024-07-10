use std::collections::HashMap;

use axum::{routing::get, Router};

pub mod test;

type PlainParams = HashMap<String, String>;

pub fn router() -> Router {
    Router::new()
        .route("/test/timeout", get(test::timeout))
        .route("/test/lorem", get(test::lorem))
}
