use axum::{extract::Query, Json};
use lipsum::lipsum;
use serde_json::{json, Value};

use super::PlainParams;

/// Returns after a specified delay.
#[utoipa::path(
    get,
    path = "/test/timeout",
    responses(
        (status = 200, description = "Timed out successfully"),
        (status = 400, description = "Invalid request")
    ),
    params(
        ("t" = u64, description = "Time to wait in seconds", nullable = false)
    )
)]
pub async fn timeout(Query(params): Query<PlainParams>) -> Json<Value> {
    let t = params.get("t").unwrap().parse::<u64>().unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(t)).await;
    json!({
        "t" : t
    }).into()
}

/// Generates a random lorem ipsum text with a specified number of words.
/// Optionally waits for a specified time before responding.
#[utoipa::path(
    get,
    path = "/test/lorem",
    responses(
        (status = 200, description = "Timed out successfully, returned lorem ipsum text"),
        (status = 400, description = "Invalid request")
    ),
    params(
        ("words" = usize, description = "Number of words to generate", nullable = true),
        ("t" = u64, description = "Time to wait in seconds", nullable = true)
    )
)]
pub async fn lorem(Query(params): Query<PlainParams>) -> Json<Value> {
    let words = params.get("words").unwrap_or(&"50".to_string()).parse::<usize>().unwrap();
    let t = params.get("t").unwrap_or(&"0".to_string()).parse::<u64>().unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(t)).await;
    json!({
        "message": lipsum(words),
        "t": t
    }).into()
}
