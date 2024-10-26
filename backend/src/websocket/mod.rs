use crate::api::auth::google::GoogleUserInfo;
use crate::AppState;
use axum::extract::ws::WebSocket;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::any;
use futures_util::StreamExt;
use log::{info, warn};
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
/// The Google ID is used to identify the user, while the WebSocket ID is used to identify the
/// connection.
///
/// Thanks to the Google ID, the server can send messages to a specific user, even if they have
/// multiple connections.
type WsUserId = (GoogleId, ConnectionId);

#[derive(Debug)]
pub enum SystemMessage {
    RemainingRequestsUpdate(u16),
    ReportDone(()),
    AddPeer((WsUserId, Sender<ServiceToClientMessage>)),
    RemovePeer(ConnectionId),
}

pub fn router() -> axum::Router<crate::AppState> {
    axum::Router::new().route("/connect", any(websocket_handler))
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    ConnectInfo(socket_info): ConnectInfo<SocketAddr>,
    google_user_info: GoogleUserInfo,
) -> impl IntoResponse {
    ws.on_upgrade(move |ws| handle_socket(ws, state.system_tx, socket_info, google_user_info))
}

async fn handle_socket(
    mut socket: WebSocket,
    system_tx: Sender<SystemMessage>,
    socket_addr: SocketAddr,
    user_info: GoogleUserInfo,
) {
    info!("New WebSocket connection: {:?}", socket_addr);

    dbg!(&user_info);

    let user_ws_id_pair = (user_info.sub().to_string(), generate_user_id());

    let (tx, mut rx) = tokio::sync::mpsc::channel::<ServiceToClientMessage>(100);

    system_tx
        .send(SystemMessage::AddPeer((user_ws_id_pair, tx)))
        .await
        .unwrap();

    tokio::select! {
        ws_msg_res = socket.next() => {
            if let Some(msg) = ws_msg_res {
                let msg = msg.unwrap();
                type Message = axum::extract::ws::Message;
                match msg {
                    Message::Close(_) => {
                        warn!("Closing connection");
                        system_tx
                            .send(SystemMessage::RemovePeer(user_info.sub().to_string()))
                            .await
                            .unwrap();
                        return;
                    }
                    _ => {
                        warn!("Received message: {:?}", msg);
                        socket.send(msg).await.unwrap();
                    }
                };
            }
        },
        service_msg_res = rx.recv() => {
            if let Some(msg) = service_msg_res {
                warn!("Received message from the main service: {:?}", msg);
            }
        }
    }
}

pub async fn start_service(
    mut system_rx: Receiver<SystemMessage>,
    cancellation_token: CancellationToken,
) -> anyhow::Result<()> {
    let mut peers = PeersMap::new();

    loop {
        tokio::select! {
            msg = system_rx.recv() => {
                if let Some(msg) = msg {
                    info!("Received system message: {:?}", msg);
                    match msg {
                        SystemMessage::AddPeer(ws_user_id) => {
                            peers.add_peer(ws_user_id);
                        }
                        SystemMessage::RemovePeer(connection_id) => {
                            peers.remove_peer(connection_id);
                        }
                        _ => {}
                    }
                }
            }
            _ = cancellation_token.cancelled() => {
                info!("WebSocket service shut down");
                break;
            }
        }
    }
    Ok(())
}
