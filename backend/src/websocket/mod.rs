use futures_util::stream::FusedStream;
use futures_util::StreamExt;
use log::{info, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::WebSocketStream;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum SystemMessage {
    RemainingRequestsUpdate(u16),
    ReportDone(()),
}

#[derive(Debug, Error)]
pub enum WebSocketServiceError {
    #[error("Authentication timeout")]
    AuthTimeout,

    #[error("Failed to authenticate: {0}")]
    AuthError(String),

    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
}

type PeersMap = Arc<Mutex<HashMap<String, WebSocketStream<TcpStream>>>>;
const AUTH_TIMEOUT_SECS: u64 = 10;

async fn authenticate_new_connection(
    ws: &mut WebSocketStream<TcpStream>,
) -> Result<String, WebSocketServiceError> {
    let msg = ws
        .next()
        .await
        .ok_or_else(|| WebSocketServiceError::AuthError("No message in sink".to_string()))??
        .to_string();
    warn!("Authenticating: {}", msg);
    // TODO: Verify JWT
    Ok(msg)
}

async fn accept_websocket_connection(
    (stream, socket_addr): (TcpStream, std::net::SocketAddr),
    peers: PeersMap,
) -> Result<(), WebSocketServiceError> {
    info!("New connection from: {}", socket_addr.to_string());
    let mut ws_stream = tokio_tungstenite::accept_async(stream).await?;

    // Authenticate the new connection within a timeout of AUTH_TIMEOUT_SECS seconds
    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_secs(AUTH_TIMEOUT_SECS)) => {
            warn!("Authentication timeout");
            Err(WebSocketServiceError::AuthTimeout)
        },
        auth_res = authenticate_new_connection(&mut ws_stream) => {
            if let Ok(email) = auth_res {
                info!("Authenticated email {email}");
                let mut peers = peers.lock().unwrap();
                peers.insert(email, ws_stream);
                info!("Peers: {}", peers.len());
            } else {
                warn!("Failed to authenticate: {}", auth_res.unwrap_err());
            }
            Ok(())
        }
    }
}

async fn handle_system_message(msg: SystemMessage) {
    match msg {
        SystemMessage::RemainingRequestsUpdate(limit) => {
            info!("New request limit: {}", limit);
        }
        SystemMessage::ReportDone(()) => {
            info!("Report done");
        }
    }
}

async fn peer_cleanup_task(peers: PeersMap, cancellation_token: CancellationToken) {
    info!("Starting peer cleanup task");

    loop {
        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                info!("Peer cleanup started");
                let mut removed = 0;
                let mut peers_lock = peers.lock().expect("Failed to lock peers map");

                let to_remove: Vec<String> = peers_lock
                    .iter()
                    .filter_map(|(email, ws)| match ws.is_terminated() {
                        true => Some(email.clone()),
                        _ => None,
                    })
                    .collect();

                for email in to_remove {
                    peers_lock.remove(&email);
                    removed += 1;
                }

                info!("Removed {} peers", removed);
            },
            _ = cancellation_token.cancelled() => {
                info!("Peer cleanup task shutting down");
                return;
            }
        }
    }
}

pub async fn start_service(
    port: u16,
    mut rx: Receiver<SystemMessage>,
    cancellation_token: CancellationToken,
) -> anyhow::Result<()> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    let peers: PeersMap = Arc::new(Mutex::new(HashMap::new()));

    info!("WebSocket service running on: {}", addr);

    // spawn a task for peer cleanup
    tokio::spawn(peer_cleanup_task(peers.clone(), cancellation_token.clone()));

    // Until cancellation, accept new WS connections and handle system messages
    loop {
        tokio::select! {
           res = listener.accept() => {
               info!("Connection accepted");
               if let Ok((stream, socket_addr)) = res {
                   info!("Connection from: {}", socket_addr.to_string());
                   let _ = accept_websocket_connection((stream, socket_addr), peers.clone()).await;
               }
           },
           msg = rx.recv() => {
               info!("Received message: {:?}", msg);
               if let Some(msg) = msg {
                   handle_system_message(msg).await;
               }
           }
           _ = cancellation_token.cancelled() => {
               break;
           },
        }
    }

    Ok(())
}
