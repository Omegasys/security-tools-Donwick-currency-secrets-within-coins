/// HTTP traffic mimic placeholder
pub struct HttpMimic;

impl HttpMimic {
    pub fn new() -> Self {
        Self
    }

    pub fn wrap(&self, data: &[u8]) -> Vec<u8> {
        let mut out = b"POST /api HTTP/1.1\r\n\r\n".to_vec();
        out.extend_from_slice(data);
        out
    }

    pub fn unwrap(&self, data: &[u8]) -> Vec<u8> {
        // naive stub extraction
        if let Some(pos) = data.windows(4).position(|w| w == b"\r\n\r\n") {
            data[pos + 4..].to_vec()
        } else {
            data.to_vec()
        }
    }
}
