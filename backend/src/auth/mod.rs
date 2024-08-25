use crate::AppState;
use axum::{routing::post, Router};

pub mod error;
mod google;
pub mod jwt;
pub(super) mod login;
pub mod middleware;

// used for OpenAPI generation, maybe not picked up by the compiler as "used".
#[allow(unused)]
pub use login::login;

/// Defines routes for the /auth path.
pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(login::login))
}
