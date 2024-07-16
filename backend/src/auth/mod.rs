use axum::Router;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
}
