use sha2::{Sha256, Digest};

/// Derive the next ratchet key using SHA-256
pub fn derive_next_key(current_key: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(current_key);
    let result = hasher.finalize();
    let mut next_key = [0u8; 32];
    next_key.copy_from_slice(&result);
    next_key
}
