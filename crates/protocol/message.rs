use serde::{Serialize, Deserialize};

/// Basic message types for the protocol
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Ping,
    Pong,
    Transaction(Vec<u8>),
    Block(Vec<u8>),
    Handshake { node_id: String, version: String },
}

impl Message {
    pub fn is_control(&self) -> bool {
        matches!(self, Message::Ping | Message::Pong | Message::Handshake { .. })
    }
}
