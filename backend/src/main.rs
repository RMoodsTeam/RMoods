use crate::db::db_client::DbClient;
use crate::env::DATABASE_URL;
use crate::fetcher::fetcher::RMoodsFetcher;
use crate::fetcher::reddit::connection::RedditConnection;
use crate::nlp::nlp_client::NlpClient;
use crate::open_api::ApiDoc;
use crate::startup::{setup_environment, shutdown_signal, verify_environment};
use crate::websocket::SystemMessage;
use axum::Router;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use reqwest::Client;
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use websocket::ws_service;

mod api;
mod app_error;
mod auth;
mod db;
mod env;
mod fetcher;
mod logging;
mod nlp;
mod open_api;
mod report;
mod startup;
mod util;
mod validation;
mod websocket;

/// State to be shared between all routes.
///
/// Contains common resources that shouldn't be created over and over again.
#[derive(Clone, Debug)]
pub struct AppState {
    pub fetcher: RMoodsFetcher,
    pub db: DbClient,
    pub http: Client,
    pub nlp_client: NlpClient,
    pub system_tx: tokio::sync::mpsc::Sender<SystemMessage>,
}

/// Run the server, assuming the environment has been already validated.
async fn run() -> anyhow::Result<()> {
    let db_url = std::env::var(DATABASE_URL).expect("DB_URL is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    log::info!(
        "Connected to the database at {}",
        if db_url.contains("localhost") {
            "localhost"
        } else {
            "remote"
        }
    );

    let db = DbClient::new(pool);

    let http = reqwest::ClientBuilder::new()
        .user_agent("RMoods")
        .redirect(RedditConnection::redirect_policy()) // to prevent redirects in case of subreddit not found
        // TODO: Abstract this away, put HTTP inside the RedditConnection
        .build()?;
    let fetcher = RMoodsFetcher::new(http.clone()).await?;
    log::info!("Connected to Reddit");

    let nlp_client = NlpClient::new();

    log::info!("Starting the WebSocket service");
    let cancellation_token = tokio_util::sync::CancellationToken::new();

    let (system_tx, system_rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(ws_service::start_service(
        system_rx,
        cancellation_token.clone(),
    ));

    let state = AppState {
        fetcher,
        db,
        http,
        nlp_client,
        system_tx,
    };

    // Allow browsers to use GET and PUT from any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Add logging
    let tracing = TraceLayer::new_for_http();

    let authorization = axum::middleware::from_fn(auth::middleware::authorization);
    let response_logging = axum::middleware::from_fn(logging::response_logging_middleware);
    // Routes after the layers won't have the layers applied
    // Example: /auth routes won't have the authorization layer, but /api will
    let app = Router::<AppState>::new()
        .nest("/api", api::router())
        .nest("/ws", websocket::router())
        .layer(authorization)
        .nest("/auth", api::auth::router())
        .with_state(state)
        .layer(tracing)
        .layer(cors)
        .layer(response_logging)
        .merge(SwaggerUi::new("/doc/ui").url("/doc/api.json", ApiDoc::openapi()))
        .into_make_service_with_connect_info::<SocketAddr>();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8001".to_string());
    // Listen on all addresses
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    log::info!("Started the RMoods server at {}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(cancellation_token))
        .await?;

    Ok(())
}

/// Entry point of the RMoods server.
/// Sets up and validates the environment, initializes the server and runs it.
#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "0");
    env_logger::init();

    setup_environment();

    if !verify_environment() {
        log::error!("Invalid environment, aborting.");
        std::process::exit(1);
    }
    log::info!("Environment OK");

    let res = run().await;
    if let Err(e) = res {
        log::error!("{e}");
        std::process::exit(1);
    }
}
