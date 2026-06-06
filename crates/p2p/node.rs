use std::collections::HashMap;
use crate::peer::Peer;
use crate::gossip::broadcast;
use crate::routing::RouteTable;

/// Represents a P2P node
pub struct Node {
    pub id: String,
    pub peers: HashMap<String, Peer>,
    pub routing: RouteTable,
}

impl Node {
    pub fn new(id: &str) -> Self {
        Node {
            id: id.to_string(),
            peers: HashMap::new(),
            routing: RouteTable::new(),
        }
    }

    /// Add a peer to the node
    pub fn add_peer(&mut self, peer: Peer) {
        self.routing.add_route(&peer.id);
        self.peers.insert(peer.id.clone(), peer);
    }

    /// Broadcast a message to all peers
    pub fn broadcast_message(&self, message: &[u8]) {
        broadcast(&self.peers, message);
    }
}
