/// WebSocket traffic mimic placeholder
pub struct WebSocketMimic;

impl WebSocketMimic {
    pub fn new() -> Self {
        Self
    }

    pub fn wrap(&self, data: &[u8]) -> Vec<u8> {
        let mut out = b"WS_FRAME:".to_vec();
        out.extend_from_slice(data);
        out
    }

    pub fn unwrap(&self, data: &[u8]) -> Vec<u8> {
        let prefix = b"WS_FRAME:";
        if data.starts_with(prefix) {
            data[prefix.len()..].to_vec()
        } else {
            data.to_vec()
        }
    }
}
