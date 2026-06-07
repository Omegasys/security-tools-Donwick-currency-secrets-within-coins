use crate::crypto::hashing::hash_bytes;

/// Represents a transfer or state change in Axiom Coin
#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
    pub payload: Option<Vec<u8>>, // encrypted coin metadata (important for your system)
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64, nonce: u64, payload: Option<Vec<u8>>) -> Self {
        let mut tx = Self {
            id: String::new(),
            from,
            to,
            amount,
            nonce,
            payload,
        };

        tx.id = tx.compute_id();
        tx
    }

    /// Deterministic transaction ID (can later be upgraded to signature hash)
    pub fn compute_id(&self) -> String {
        let raw = format!("{}{}{}{}", self.from, self.to, self.amount, self.nonce);
        hex::encode(hash_bytes(raw.as_bytes()))
    }
}
