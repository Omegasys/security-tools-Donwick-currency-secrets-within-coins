/// DNS traffic mimic placeholder
pub struct DnsMimic;

impl DnsMimic {
    pub fn new() -> Self {
        Self
    }

    pub fn wrap(&self, data: &[u8]) -> Vec<u8> {
        let mut out = vec![0x01, 0x00]; // fake header
        out.extend_from_slice(data);
        out
    }

    pub fn unwrap(&self, data: &[u8]) -> Vec<u8> {
        if data.len() > 2 {
            data[2..].to_vec()
        } else {
            data.to_vec()
        }
    }
}
