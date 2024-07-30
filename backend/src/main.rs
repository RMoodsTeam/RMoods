use axum::{routing::get, Json, Router};
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use log::{info, warn};
use reddit::connection::RedditConnection;
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod app_error;
mod auth;
mod reddit;

/// OpenAPI documentation for the RMoods server
#[derive(OpenApi)]
#[openapi(paths(hello, api::debug::lorem, api::debug::timeout))]
struct ApiDoc;

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

#[derive(Clone)]
pub struct AppState {
    pub reddit: RedditConnection,
    pub pool: Pool<Postgres>,
    pub http: Client,
}

/// Entry point of the RMoods server.
/// Initializes the HTTP server and runs it.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "0");
    env_logger::init();

    if dotenvy::dotenv().is_err() {
        warn!(".env not found. Environment variables will have to be defined outside of .env");
    }

    let url = std::env::var("DATABASE_URL").expect("DB_URL is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await?;
    info!("Connected to the database");

    let http = reqwest::ClientBuilder::new().user_agent("RMoods").build()?;
    let reddit = RedditConnection::new(http.clone()).await?;
    info!("Connected to Reddit");

    let state = AppState { reddit, pool, http };

    // Allow browsers to use GET and PUT from any origin
    let cors =
        CorsLayer::new()
            .allow_origin(Any)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Add logging
    let tracing = TraceLayer::new_for_http();

    let authorization = axum::middleware::from_fn(auth::middleware::authorization);

    // Routes after the layers won't have the layers applied
    let app = Router::<AppState>::new()
        .route("/", get(hello))
        .nest("/api", api::router())
        .layer(authorization)
        .nest("/auth", auth::router())
        .with_state(state)
        .layer(tracing)
        .layer(cors)
        .merge(SwaggerUi::new("/doc/ui").url("/doc/api.json", ApiDoc::openapi()));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8001".to_string());
    // Listen on all addresses
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Started the RMoods server at {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
