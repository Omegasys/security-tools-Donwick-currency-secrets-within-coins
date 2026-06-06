use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM crate
use aes_gcm::aead::{Aead, NewAead};

/// Encrypt plaintext using AES-256-GCM
pub fn encrypt(key_bytes: &[u8; 32], nonce_bytes: &[u8; 12], plaintext: &[u8]) -> Vec<u8> {
    let key = Key::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.encrypt(nonce, plaintext).expect("encryption failure")
}

/// Decrypt ciphertext using AES-256-GCM
pub fn decrypt(key_bytes: &[u8; 32], nonce_bytes: &[u8; 12], ciphertext: &[u8]) -> Vec<u8> {
    let key = Key::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ciphertext).expect("decryption failure")
}
