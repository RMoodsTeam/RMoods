use axum::{
    http::Method, routing::get, Json, Router
};
use log::info;
use reqwest::Client;
use serde_json::{json, Value};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod reddit;

/// OpenAPI documentation for the RMoods server
#[derive(OpenApi)]
#[openapi(paths(hello))]
struct ApiDoc;

/// The port on which the server starts.
const PORT: u16 = 3000;

lazy_static::lazy_static! {
    /// The Reddit Connection used to aggregate clients and use their tokens
    static ref REDDIT_CONNECTION: reddit::RedditConnection = reddit::RedditConnection::new();
    static ref REQWEST_CLIENT: Client = reqwest::ClientBuilder::new().user_agent("RMoods").build().unwrap();
}

/// Returns a welcome message and a link to our documentation
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Always"),
    ),
)]
async fn hello() -> Json<Value> {
    json!({
        "message" : "Welcome to the RMoods Backend!",
        "docs": "https://rmoodsteam.github.io/RMoods/backend/rmoods_backend/index.html",
    })
    .into()
}

/// Entry point of the RMoods server.
/// Initializes the HTTP server and runs it.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "0");
    env_logger::init();
    
    // Allow browsers to use GET and PUT from any origin
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::PUT])
        .allow_origin(Any);

    // Add logging
    let tracing = TraceLayer::new_for_http();

    let app = Router::new()
        .route("/", get(hello))
        .nest("/auth", auth::router())
        .layer(tracing)
        .layer(cors)
        .merge(SwaggerUi::new("/doc/ui").url("/doc/api.json", ApiDoc::openapi()));

    // Listen on all addresses
    let addr = format!("0.0.0.0:{PORT}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    for client in REDDIT_CONNECTION.clients.iter()  {
        info!("{:?}", client.0.fetch_access_token().await)
    }

    info!("Starting the RMoods server at {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_test() {
        assert_eq!(2 + 2, 4);
    }
}
