use crate::api::auth::google::GoogleUserInfo;
use crate::websocket;
use crate::websocket::{ClientMessage, SystemMessage};
use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use serde_json::json;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

/// Handles a new WebSocket connection.
/// One client (one Google account) can have many active connections at once.
/// The server uses the Google ID to identify the user, and the WebSocket ID to identify the connection.
///
/// The handler can communicate with the WebSocket Service through `mpsc` channels.
/// 1. `system_tx` channel is the app-wide channel to send messages to the WebSocket Service.
/// 2. `service_to_client_tx` channel is the chanel where the Service sends messages to the client handlers.
pub async fn handle_socket(
    mut socket: WebSocket,
    system_tx: Sender<SystemMessage>,
    socket_addr: SocketAddr,
    user_info: GoogleUserInfo,
) {
    log::info!("New WebSocket connection: {:?}", socket_addr);
    dbg!(&user_info);

    let user_ws_id_pair = (user_info.sub().to_string(), websocket::generate_user_id());

    let (service_to_client_tx, mut service_to_client_rx) =
        tokio::sync::mpsc::channel::<ClientMessage>(100);

    // Register the connection. We give the Service our tx, so it can call the handler when needed.
    let res = system_tx
        .send(SystemMessage::AddPeer((
            user_ws_id_pair.clone(),
            service_to_client_tx,
        )))
        .await;

    if let Err(e) = res {
        log::error!("Failed to register the new peer: {:?}", e);
        return;
    }

    // Cleanup function to remove the peer from the system
    let ask_to_remove_this_peer = || async {
        let res = system_tx
            .send(SystemMessage::RemovePeer(user_ws_id_pair.1))
            .await;
        if let Err(e) = res {
            log::error!("Failed to remove the peer: {:?}", e);
        }
    };

    loop {
        tokio::select! {
            ws_msg_res = socket.next() => match ws_msg_res {
                Some(Ok(msg)) => match msg {
                    axum::extract::ws::Message::Close(_) => {
                        log::info!("Closing connection");
                        ask_to_remove_this_peer().await;
                        return;
                    }
                    _ => {
                        log::info!("Received message: {:?}. Echoing", msg);
                        let _ = socket.send(msg).await;
                    }
                },
                Some(Err(e)) => {
                    log::warn!("Error receiving message: {:?}", e);
                    ask_to_remove_this_peer().await;
                    return;
                }
                None => {
                    log::warn!("Connection closed - WS stream ended");
                    ask_to_remove_this_peer().await;
                    return;
                }
            },
            service_msg_res = service_to_client_rx.recv() => {
                if let Some(msg) = service_msg_res {
                    log::debug!("Received message from the main service: {:?}", msg);
                    let _ = socket.send(Message::Text(json!(msg).to_string())).await;
                } else {
                    log::error!("WS Service task has exited or closed the mpsc channel");
                    return;
                }
            }
        }
    }
}
