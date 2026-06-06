use sha2::{Sha256, Digest};

/// Compute a simple address from a public key
pub fn public_key_to_address(pubkey: &[u8]) -> String {
    let hash = Sha256::digest(pubkey);
    hex::encode(&hash[0..20]) // first 20 bytes as address
}
