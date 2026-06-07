use super::HardwareSigner;

/// Simulated Hardware Security Module (HSM)
pub struct Hsm {
    key_id: String,
}

impl Hsm {
    pub fn new(key_id: String) -> Self {
        Self { key_id }
    }

    fn internal_sign(&self, data: &[u8]) -> Vec<u8> {
        // Placeholder: replace with real HSM API or PKCS#11 later
        let mut out = self.key_id.as_bytes().to_vec();
        out.extend_from_slice(data);
        out
    }
}

impl HardwareSigner for Hsm {
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.internal_sign(data)
    }

    fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        let expected = self.internal_sign(data);
        expected == signature
    }
}
