use crate::websocket::{ConnectionId, ServiceToClientMessage, WsUserId};
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;

/// A map of all connected peers.
///
/// Key: A tuple of the user's Google ID and the WebSocket connection ID.
/// Value: The sender half of a channel that is used to send messages to that particular connection.
///
/// Thanks to the key being a pair, the server can send messages to a specific user, even if they have
/// multiple connections. We can identify a user and all their connections.
#[derive(Debug)]
pub struct PeersMap {
    peers: HashMap<WsUserId, Sender<ServiceToClientMessage>>,
}

impl PeersMap {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub fn add_peer(&mut self, (user_id, sender): (WsUserId, Sender<ServiceToClientMessage>)) {
        self.peers.insert(user_id, sender);
    }

    /// Remove a particular WS connection from the map.
    /// This does not remove all connections for a user, only the one with the given connection ID.
    /// Called when a connection is closed.
    pub fn remove_peer(&mut self, connection_id: ConnectionId) {
        let len_before = self.peers.len();
        self.peers
            .retain(|(_, conn_id), _| conn_id != &connection_id);
        let len_after = self.peers.len();
        if len_before == len_after {
            log::error!(
                "No peer with connection ID {}. No peers removed. Each connection_id that is to be removed must come from the peer-specific handler task.",
                connection_id
            );
        }
    }

    pub fn len(&self) -> usize {
        self.peers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc::channel;

    #[tokio::test]
    async fn test_add_peer() {
        let mut peers = PeersMap::new();
        let (sender, _) = channel(1);
        let user_id = ("google_id".to_string(), "conn_id".to_string());

        peers.add_peer((user_id.clone(), sender.clone()));
        assert_eq!(peers.peers.len(), 1);
        assert!(peers.peers.get(&user_id).is_some());
    }

    #[tokio::test]
    async fn test_remove_peer() {
        let mut peers = PeersMap::new();
        let (sender, _) = channel(1);
        let user_id = ("google_id".to_string(), "conn_id".to_string());

        peers.add_peer((user_id.clone(), sender));
        assert_eq!(peers.peers.len(), 1);
        peers.remove_peer("conn_id".to_string());
        assert_eq!(peers.peers.len(), 0);
    }

    #[tokio::test]
    async fn test_remove_peer_no_match() {
        let mut peers = PeersMap::new();
        let (sender, _) = channel(1);
        let user_id = ("google_id".to_string(), "conn_id".to_string());

        peers.add_peer((user_id.clone(), sender));
        assert_eq!(peers.peers.len(), 1);
        peers.remove_peer("conn_id_2".to_string());
        assert_eq!(peers.peers.len(), 1);
    }

    #[tokio::test]
    async fn test_remove_peer_empty() {
        let mut peers = PeersMap::new();
        peers.remove_peer("conn_id".to_string());
        assert_eq!(peers.peers.len(), 0);
    }

    #[tokio::test]
    async fn test_remove_peer_multiple() {
        let mut peers = PeersMap::new();
        let (sender, _) = channel(1);
        let user_id = ("google_id".to_string(), "conn_id".to_string());
        let user_id_2 = ("google_id_2".to_string(), "conn_id_2".to_string());

        peers.add_peer((user_id.clone(), sender.clone()));
        peers.add_peer((user_id_2.clone(), sender.clone()));
        assert_eq!(peers.peers.len(), 2);
        peers.remove_peer("conn_id".to_string());
        assert_eq!(peers.peers.len(), 1);
        assert!(peers.peers.get(&user_id_2).is_some());
    }
}
