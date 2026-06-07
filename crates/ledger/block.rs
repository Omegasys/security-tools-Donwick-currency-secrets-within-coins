use crate::ledger::transaction::Transaction;
use crate::crypto::hashing::hash_bytes;

/// A block in the ledger chain
#[derive(Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        timestamp: u64,
        transactions: Vec<Transaction>,
        nonce: u64,
    ) -> Self {
        let mut block = Self {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce,
            hash: String::new(),
        };

        block.hash = block.compute_hash();
        block
    }

    pub fn compute_hash(&self) -> String {
        let mut data = String::new();

        data.push_str(&self.index.to_string());
        data.push_str(&self.previous_hash);
        data.push_str(&self.timestamp.to_string());
        data.push_str(&self.nonce.to_string());

        for tx in &self.transactions {
            data.push_str(&tx.id);
        }

        hex::encode(hash_bytes(data.as_bytes()))
    }
}
