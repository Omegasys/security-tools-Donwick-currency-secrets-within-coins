use std::collections::HashMap;
use crate::peer::Peer;

/// Broadcast a message to all peers
pub fn broadcast(peers: &HashMap<String, Peer>, message: &[u8]) {
    for (id, peer) in peers {
        println!("Sending message to {}@{}: {:?}", id, peer.address, message);
        // Here you would implement real network send logic
    }
}
