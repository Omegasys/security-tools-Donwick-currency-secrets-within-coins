#[cfg(test)]
mod tests {
    use axiom_protocol::message::Message;

    #[test]
    fn test_message_serialization() {
        let msg = Message::new("ping", vec![1, 2, 3]);
        let serialized = msg.serialize();
        let deserialized = Message::deserialize(&serialized);
        assert_eq!(msg.msg_type, deserialized.msg_type);
        assert_eq!(msg.payload, deserialized.payload);
    }
}
