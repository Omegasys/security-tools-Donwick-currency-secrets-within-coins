use crate::crypto::hashing::hash_bytes;

/// Software fallback signing (non-hardware)
pub struct SoftwareSigner {
    pub private_key: Vec<u8>,
}

impl SoftwareSigner {
    pub fn new(private_key: Vec<u8>) -> Self {
        Self { private_key }
    }

    fn sign_internal(&self, data: &[u8]) -> Vec<u8> {
        let mut input = self.private_key.clone();
        input.extend_from_slice(data);
        hash_bytes(&input).to_vec()
    }
}

impl super::HardwareSigner for SoftwareSigner {
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.sign_internal(data)
    }

    fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        self.sign_internal(data) == signature
    }
}
