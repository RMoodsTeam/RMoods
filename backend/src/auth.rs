pub mod router;

use axum::Router;

pub fn router() -> Router {
    Router::new()
}
