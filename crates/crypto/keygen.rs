use rand::RngCore;

/// Generate a random 32-byte key (for AES, etc.)
pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}
