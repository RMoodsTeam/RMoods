use axum::{routing::post, Router};
use crate::AppState;

pub mod jwt;
mod routes;
pub mod error;
mod google;
pub mod middleware;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(routes::login))
}
