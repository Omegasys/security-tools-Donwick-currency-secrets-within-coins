use crate::key_schedule::derive_next_key;

/// Maintains the current ratchet state
#[derive(Debug, Clone)]
pub struct RatchetState {
    pub current_key: [u8; 32],
}

impl RatchetState {
    pub fn new(initial_key: [u8; 32]) -> Self {
        RatchetState { current_key: initial_key }
    }

    /// Advance the ratchet state to a new key
    pub fn ratchet_forward(&mut self) {
        self.current_key = derive_next_key(&self.current_key);
    }
}
