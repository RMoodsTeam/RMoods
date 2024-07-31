use axum::{routing::post, Router};
use crate::AppState;

pub mod jwt;
pub(super) mod login;
pub mod error;
mod google;
pub mod middleware;

// used for OpenAPI generation, maybe not picked up by the compiler as used
#[allow(unused)] 
pub use login::login;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(login::login))
}

