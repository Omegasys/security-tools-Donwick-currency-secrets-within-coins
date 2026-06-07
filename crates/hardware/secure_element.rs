use super::HardwareSigner;

/// Simulated secure element chip (SIM/TEE-like)
pub struct SecureElement {
    device_id: u64,
}

impl SecureElement {
    pub fn new(device_id: u64) -> Self {
        Self { device_id }
    }

    fn sign_internal(&self, data: &[u8]) -> Vec<u8> {
        let mut out = self.device_id.to_le_bytes().to_vec();
        out.extend_from_slice(data);
        out
    }
}

impl HardwareSigner for SecureElement {
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.sign_internal(data)
    }

    fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        self.sign_internal(data) == signature
    }
}
