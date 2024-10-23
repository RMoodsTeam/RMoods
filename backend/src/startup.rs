use tokio::signal;

/// Ensure that all necessary environment variables are available at server startup.
/// It's important to keep this updated as our .env file grows.
pub fn verify_environment() -> bool {
    let needed_vars = [
        "CLIENT_ID",
        "CLIENT_SECRET",
        "DATABASE_URL",
        "JWT_SECRET",
        "GOOGLE_CLIENT_ID",
        "GOOGLE_CLIENT_SECRET",
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

pub async fn shutdown_signal() {
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
            log::info!("Received Ctrl+C signal, shutting down");
        },
        _ = terminate => {},
    }
}
