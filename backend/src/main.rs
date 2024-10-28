use crate::open_api::ApiDoc;
use crate::reddit_fetcher::fetcher::RMoodsFetcher;
use crate::startup::{shutdown_signal, verify_environment};
use crate::websocket::SystemMessage;
use api::auth;
use axum::Router;
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use log::{error, info, warn};
use reqwest::Client;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod app_error;
mod open_api;
mod reddit_fetcher;
mod startup;
mod websocket;

/// State to be shared between all routes.
///
/// Contains common resources that shouldn't be created over and over again.
#[derive(Clone)]
pub struct AppState {
    pub fetcher: RMoodsFetcher,
    pub pool: Pool<Postgres>,
    pub http: Client,
    pub system_tx: tokio::sync::mpsc::Sender<SystemMessage>,
}

/// Run the server, assuming the environment has been already validated.
async fn run() -> anyhow::Result<()> {
    let url = std::env::var("DATABASE_URL").expect("DB_URL is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await?;
    info!("Connected to the database");

    let http = reqwest::ClientBuilder::new().user_agent("RMoods").build()?;
    let fetcher = RMoodsFetcher::new(http.clone()).await?;
    info!("Connected to Reddit");

    info!("Starting the WebSocket service");
    let cancellation_token = tokio_util::sync::CancellationToken::new();

    let (system_tx, system_rx) = tokio::sync::mpsc::channel::<SystemMessage>(100);
    tokio::spawn(websocket::start_service(
        system_rx,
        cancellation_token.clone(),
    ));

    let state = AppState {
        fetcher,
        pool,
        http,
        system_tx,
    };

    // Allow browsers to use GET and PUT from any origin
    let cors =
        CorsLayer::new()
            .allow_origin(Any)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Add logging
    let tracing = TraceLayer::new_for_http();

    let authorization = axum::middleware::from_fn(auth::middleware::authorization);

    // Routes after the layers won't have the layers applied
    // Example: /auth routes won't have the authorization layer, but /api will
    let app = Router::<AppState>::new()
        .nest("/api", api::router())
        .nest("/ws", websocket::router())
        .layer(authorization)
        .nest("/auth", auth::router())
        .with_state(state)
        .layer(tracing)
        .layer(cors)
        .merge(SwaggerUi::new("/doc/ui").url("/doc/api.json", ApiDoc::openapi()))
        .into_make_service_with_connect_info::<SocketAddr>();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8001".to_string());
    // Listen on all addresses
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Started the RMoods server at {}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(cancellation_token))
        .await?;

    Ok(())
}

/// Entry point of the RMoods server.
/// Validates the environment, initializes the server and runs it.
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "0");
    env_logger::init();

    if dotenvy::dotenv().is_err() {
        warn!(".env not found. Environment variables will have to be defined outside of .env");
    }

    if !verify_environment() {
        error!("Invalid environment, aborting.");
        std::process::exit(1);
    }
    info!("Environment OK");

    let res = run().await;
    if let Err(e) = res {
        log::error!("{e}");
        std::process::exit(1);
    }
}
