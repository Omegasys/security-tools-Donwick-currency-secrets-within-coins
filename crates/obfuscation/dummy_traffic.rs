use super::ObfuscationLayer;

/// Generates or passes-through dummy traffic patterns (placeholder)
pub struct DummyTraffic {
    pub enabled: bool,
}

impl DummyTraffic {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    fn mix_dummy(&self, data: &[u8]) -> Vec<u8> {
        if !self.enabled {
            return data.to_vec();
        }

        let mut out = Vec::with_capacity(data.len() + 8);
        out.extend_from_slice(&[0, 0, 0, 0]); // fake prefix
        out.extend_from_slice(data);
        out.extend_from_slice(&[0xAA, 0xAA]); // fake suffix
        out
    }
}

impl ObfuscationLayer for DummyTraffic {
    fn process_outbound(&self, data: &[u8]) -> Vec<u8> {
        self.mix_dummy(data)
    }

    fn process_inbound(&self, data: &[u8]) -> Vec<u8> {
        // strip dummy framing if present
        if data.len() > 6 {
            data[4..data.len().saturating_sub(2)].to_vec()
        } else {
            data.to_vec()
        }
    }
}
