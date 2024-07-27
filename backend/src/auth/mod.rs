use axum::{routing::post, Router};
use crate::AppState;

mod jwt;
mod routes;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(routes::login))
}
