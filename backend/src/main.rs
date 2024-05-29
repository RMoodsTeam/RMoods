use axum::{
    http::Method, routing::get, Json, Router
};
use log::info;
use serde_json::{json, Value};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod auth;

/// OpenAPI documentation for the XMoods server
#[derive(OpenApi)]
#[openapi(paths(hello))]
struct ApiDoc;

/// The port on which the server starts.
const PORT: u16 = 3000;

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
        "message" : "Welcome to the XMoods Backend!",
        "docs": "https://xmoods.github.io/XMoods/backend/xmoods_backend/index.html",
    })
    .into()
}

/// Entry point of the XMoods server.
/// Initializes the HTTP server and runs it.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
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

    info!("Starting the XMoods server at {}", addr);
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
