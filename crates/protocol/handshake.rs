use crate::message::Message;

/// Simple handshake structure
pub struct Handshake;

impl Handshake {
    pub fn create(node_id: &str, version: &str) -> Message {
        Message::Handshake { node_id: node_id.to_string(), version: version.to_string() }
    }

    pub fn is_valid(msg: &Message) -> bool {
        match msg {
            Message::Handshake { version, .. } => version == crate::PROTOCOL_VERSION,
            _ => false,
        }
    }
}
