use crate::env::*;
use log::info;
use tokio::signal;
use tokio_util::sync::CancellationToken;

/// Ensure that all necessary environment variables are available at server startup.
/// It's important to keep this updated as our .env file grows.
pub fn verify_environment() -> bool {
    let needed_vars = [
        CLIENT_ID,
        CLIENT_SECRET,
        DATABASE_URL,
        JWT_SECRET,
        GOOGLE_CLIENT_ID,
        GOOGLE_CLIENT_SECRET,
        NLP_URL,
        NLP_API_KEY,
    ];
    let defined: Vec<String> = std::env::vars().map(|(k, _)| k).collect();

    let mut is_ok = true;
    needed_vars
        .iter()
        .filter(|&needed| !defined.contains(&needed.to_string()))
        .for_each(|missing| {
            log::error!("{missing} is not defined in the environment.");
            is_ok = false
        });
    is_ok
}

/// Sets up the environment for the application.
/// If we're in debug mode, we load the .env file.
/// Otherwise, we expect all environment variables to be defined.
pub fn setup_environment() {
    #[cfg(debug_assertions)]
    {
        log::warn!("Running in debug mode.");
        if dotenvy::dotenv().is_err() {
            log::warn!(
                "Failed to load .env file. Environment variables will have to be defined outside"
            );
        }
    }

    #[cfg(not(debug_assertions))]
    {
        log::warn!("Running in production mode.");
        if dotenvy::from_filename_override(".env.prod").is_err() {
            log::warn!(
            "Failed to load .env.prod file. Environment variables will have to be defined outside"
        );
        }
    }
}

pub async fn shutdown_signal(cancellation_token: CancellationToken) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            log::info!("Received Ctrl+C signal, shutting down main HTTP server");
            cancellation_token.cancel();
            info!("Shutting down WebSocket service");
        },
        _ = terminate => {
            log::info!("Received SIGTERM, shutting down");
            cancellation_token.cancel();
            info!("Shutting down WebSocket service");
        },
        _ = cancellation_token.cancelled() => {
            info!("WebSocket service triggered shutdown, shutting down main HTTP server");
        },
    }
}
