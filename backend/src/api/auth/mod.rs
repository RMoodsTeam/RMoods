use crate::AppState;
use axum::{routing::post, Router};

pub(crate) mod login;

// used for OpenAPI generation, maybe not picked up by the compiler as "used".
#[allow(unused)]
pub use login::login;

/// Defines routes for the /auth path.
pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(login::login))
}
