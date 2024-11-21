use crate::api::auth::google::GoogleUserInfo;
use crate::app_error::AppError;
use crate::nlp::report::ReportID;
use crate::AppState;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Serialize;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::atomic::AtomicUsize;
use tokio::sync::mpsc::Sender;

mod peers;
mod socket_handler;
pub mod ws_service;

/// Generates a unique user ID, thread safe.
fn generate_user_id() -> String {
    static USER_ID_GEN: AtomicUsize = AtomicUsize::new(0);
    USER_ID_GEN
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        .to_string()
}

#[derive(Debug, Serialize)]
pub enum ClientMessage {
    RemainingRequestsUpdate(u16),
    ReportDone(ReportID),
    ReportError(AppError),
}

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
    ReportDone((ReportID, GoogleUserInfo)),
    ReportError((AppError, GoogleUserInfo)),
    AddPeer((WsUserId, Sender<ClientMessage>)),
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
    ws.on_failed_upgrade(|e| log::error!("Failed WebSocket upgrade: {}", e))
        .on_upgrade(move |ws| {
            socket_handler::handle_socket(ws, state.system_tx, socket_info, google_user_info)
        })
}
