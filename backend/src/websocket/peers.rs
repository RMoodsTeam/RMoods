use crate::websocket::{ConnectionId, ServiceToClientMessage, WsUserId};
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;

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

    pub fn remove_peer(&mut self, connection_id: ConnectionId) {
        self.peers
            .retain(|(_, conn_id), _| conn_id != &connection_id);
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
