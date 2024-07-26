use axum::{http::Method, routing::get, Json, Router};
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
#[openapi(paths(hello, api::test::lorem, api::test::timeout))]
struct ApiDoc;

lazy_static::lazy_static! {
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

#[derive(Clone)]
pub struct AppState {
    pub reddit: RedditConnection,
    pub pool: Pool<Postgres>,
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

    // Allow browsers to use GET and PUT from any origin
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::PUT])
        .allow_origin(Any);

    // Add logging
    let tracing = TraceLayer::new_for_http();

    let state = AppState {
        reddit: RedditConnection::new(REQWEST_CLIENT.clone()).await?,
        pool,
    };

    // Routes after the layers won't have the layers applied
    let app = Router::<AppState>::new()
        .route("/", get(hello))
        .nest("/auth", auth::router())
        .nest("/api", api::router())
        .with_state(state)
        .layer(tracing)
        .layer(cors)
        .merge(SwaggerUi::new("/doc/ui").url("/doc/api.json", ApiDoc::openapi()));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8001".to_string());
    // Listen on all addresses
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Starting the RMoods server at {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
