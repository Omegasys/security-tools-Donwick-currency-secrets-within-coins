use crate::message::Message;
use bincode;

/// Serialize a message to bytes
pub fn serialize(msg: &Message) -> Vec<u8> {
    bincode::serialize(msg).expect("Failed to serialize message")
}

/// Deserialize bytes back into a message
pub fn deserialize(data: &[u8]) -> Message {
    bincode::deserialize(data).expect("Failed to deserialize message")
}
