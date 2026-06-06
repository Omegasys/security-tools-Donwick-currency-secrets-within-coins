use crate::state::RatchetState;
use crate::key_schedule::derive_next_key;

/// Represents a secure session between two peers
pub struct Session {
    pub state: RatchetState,
}

impl Session {
    /// Create a new session with an initial key
    pub fn new(initial_key: &[u8; 32]) -> Self {
        Self {
            state: RatchetState::new(*initial_key),
        }
    }

    /// Encrypt a message (dummy implementation)
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let key = self.state.current_key;
        let ciphertext = plaintext.iter().zip(key.iter()).map(|(p, k)| p ^ k).collect();
        self.state.ratchet_forward(); // advance the ratchet
        ciphertext
    }

    /// Decrypt a message (dummy implementation)
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        let key = self.state.current_key;
        let plaintext = ciphertext.iter().zip(key.iter()).map(|(c, k)| c ^ k).collect();
        self.state.ratchet_forward(); // advance the ratchet
        plaintext
    }
}
