use crate::websocket::peers::PeersMap;
use crate::websocket::SystemMessage;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

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
                    match msg {
                        SystemMessage::AddPeer(ws_user_id) => {
                            peers.add_peer(ws_user_id);
                            log::info!("Peers number: {}", peers.len());
                        }
                        SystemMessage::RemovePeer(connection_id) => {
                            peers.remove_peer(connection_id);
                            log::info!("Peers number: {}", peers.len());
                        }
                        SystemMessage::ReportDone(report) => {
                            let str = format!("{:?}", report);
                            log::info!("Received a report from the main thread of len: {:?}", str.len());
                            // TODO: Send the report to the client
                        },
                        SystemMessage::RemainingRequestsUpdate(remaining) => {
                            log::info!("Remaining requests: {remaining}");
                            // TODO: Broadcast remaining requests to all clients
                        }
                    }
                } else {
                    log::error!("The main HTTP process has exited or closed the mpsc channel");
                    return;
                }
            }
            _ = cancellation_token.cancelled() => {
                log::info!("WebSocket service shut down");
                return;
            }
        }
    }
}
