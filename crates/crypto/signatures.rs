use ed25519_dalek::{Keypair, Signature, Signer, PublicKey, Verifier};
use rand::rngs::OsRng;

/// Generate a new Ed25519 keypair
pub fn generate_keypair() -> Keypair {
    Keypair::generate(&mut OsRng)
}

/// Sign a message
pub fn sign_message(keypair: &Keypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

/// Verify a signature
pub fn verify_signature(pubkey: &PublicKey, message: &[u8], signature: &Signature) -> bool {
    pubkey.verify(message, signature).is_ok()
}
