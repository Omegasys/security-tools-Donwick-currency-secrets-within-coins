use crate::keypair;
use ed25519_dalek::Keypair;

/// A simple wallet structure
pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    /// Create a new wallet
    pub fn new() -> Self {
        let keypair = keypair::generate_keypair();
        Self { keypair }
    }

    /// Get the wallet's public key in bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }
}
