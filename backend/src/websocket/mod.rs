use crate::api::auth::google::GoogleUserInfo;
use crate::AppState;
use axum::extract::ws::WebSocket;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use futures_util::StreamExt;
use log::{error, info, warn};
use peers::PeersMap;
use std::net::SocketAddr;
use std::sync::atomic::AtomicUsize;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::sync::CancellationToken;

mod peers;

/// Generates a unique user ID, thread safe.
fn generate_user_id() -> String {
    static USER_ID_GEN: AtomicUsize = AtomicUsize::new(0);
    USER_ID_GEN
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        .to_string()
}

#[derive(Debug)]
pub struct ServiceToClientMessage;

type ConnectionId = String;
type GoogleId = String;

/// A tuple of the user's Google ID and the WebSocket connection ID.
/// * Google ID is used to identify the user
/// * WebSocket ID is used to identify the connection.
///
/// Thanks to the Google ID, the server can send messages to a specific user, even if they have
/// multiple connections.
type WsUserId = (GoogleId, ConnectionId);

/// Messages that the WebSocket Service can receive from the main HTTP process.
#[derive(Debug)]
pub enum SystemMessage {
    RemainingRequestsUpdate(u16),
    ReportDone(()),
    AddPeer((WsUserId, Sender<ServiceToClientMessage>)),
    RemovePeer(ConnectionId),
}

/// Defines the WebSocket routes.
pub fn router() -> axum::Router<crate::AppState> {
    axum::Router::new().route("/connect", get(websocket_handler))
}

/// Handles the WebSocket upgrade request.
/// The handler is responsible for creating a new WebSocket connection and managing it.
/// Frontend calls this endpoint to establish a WebSocket connection.
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    ConnectInfo(socket_info): ConnectInfo<SocketAddr>,
    google_user_info: GoogleUserInfo,
) -> impl IntoResponse {
    ws.on_failed_upgrade(|e| error!("Failed WebSocket upgrade: {}", e))
        .on_upgrade(move |ws| handle_socket(ws, state.system_tx, socket_info, google_user_info))
}

/// Handles a new WebSocket connection.
/// One client (one Google account) can have many active connections at once.
/// The server uses the Google ID to identify the user, and the WebSocket ID to identify the connection.
///
/// The handler can communicate with the WebSocket Service through `mpsc` channels.
/// 1. `system_tx` channel is the app-wide channel to send messages to the WebSocket Service.
/// 2. `service_to_client_tx` channel is the chanel where the Service sends messages to the client handlers.
async fn handle_socket(
    mut socket: WebSocket,
    system_tx: Sender<SystemMessage>,
    socket_addr: SocketAddr,
    user_info: GoogleUserInfo,
) {
    info!("New WebSocket connection: {:?}", socket_addr);
    dbg!(&user_info);

    let user_ws_id_pair = (user_info.sub().to_string(), generate_user_id());

    let (service_to_client_tx, mut service_to_client_rx) =
        tokio::sync::mpsc::channel::<ServiceToClientMessage>(100);

    // Register the connection. We give the Service our tx, so it can call the handler when needed.
    let res = system_tx
        .send(SystemMessage::AddPeer((
            user_ws_id_pair.clone(),
            service_to_client_tx,
        )))
        .await;

    if let Err(e) = res {
        error!("Failed to register the new peer: {:?}", e);
        return;
    }

    // Cleanup function to remove the peer from the system
    let ask_to_remove_this_peer = || async {
        let res = system_tx
            .send(SystemMessage::RemovePeer(user_ws_id_pair.1))
            .await;
        if let Err(e) = res {
            error!("Failed to remove the peer: {:?}", e);
        }
    };

    loop {
        tokio::select! {
            ws_msg_res = socket.next() => match ws_msg_res {
                Some(Ok(msg)) => match msg {
                    axum::extract::ws::Message::Close(_) => {
                        info!("Closing connection");
                        ask_to_remove_this_peer().await;
                        return;
                    }
                    _ => {
                        info!("Received message: {:?}. Echoing", msg);
                        let _ = socket.send(msg).await;
                    }
                },
                Some(Err(e)) => {
                    warn!("Error receiving message: {:?}", e);
                    ask_to_remove_this_peer().await;
                    return;
                }
                None => {
                    warn!("Connection closed - WS stream ended");
                    ask_to_remove_this_peer().await;
                    return;
                }
            },
            service_msg_res = service_to_client_rx.recv() => {
                if let Some(msg) = service_msg_res {
                    warn!("Received message from the main service: {:?}", msg);
                    todo!("Handle service message");
                } else {
                    error!("WS Service task has exited or closed the mpsc channel");
                    return;
                }
            }
        }
    }
}

/// Starts and maintains the WebSocket Service.
///
/// This service is responsible for managing the WebSocket connections, and sending messages to the clients.
/// * When a new WebSocket connection is established, the service is asked to register it.
/// * When a connection is closed, the service is asked to remove it.
///
/// The service is also responsible for sending messages to the clients.
/// * When a report request completes, the service sends the report to the client.
/// * When the number of remaining Reddit API requests changes, the service sends the new number to all clients.
pub async fn start_service(
    mut system_rx: Receiver<SystemMessage>,
    cancellation_token: CancellationToken,
) {
    let mut peers = PeersMap::new();

    loop {
        tokio::select! {
            msg = system_rx.recv() => {
                if let Some(msg) = msg {
                    info!("Received system message: {:?}", msg);
                    match msg {
                        SystemMessage::AddPeer(ws_user_id) => {
                            peers.add_peer(ws_user_id);
                            info!("Peers number: {}", peers.len());
                        }
                        SystemMessage::RemovePeer(connection_id) => {
                            peers.remove_peer(connection_id);
                            info!("Peers number: {}", peers.len());
                        }
                        _ => {}
                    }
                } else {
                    error!("The main HTTP process has exited or closed the mpsc channel");
                    return;
                }
            }
            _ = cancellation_token.cancelled() => {
                info!("WebSocket service shut down");
                return;
            }
        }
    }
}
