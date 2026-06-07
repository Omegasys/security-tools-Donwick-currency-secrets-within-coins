use std::collections::HashMap;
use crate::ledger::transaction::Transaction;

/// Simple account/balance state model
#[derive(Clone, Debug)]
pub struct LedgerState {
    pub balances: HashMap<String, u64>,
    pub nonces: HashMap<String, u64>,
}

impl LedgerState {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            nonces: HashMap::new(),
        }
    }

    pub fn get_balance(&self, addr: &str) -> u64 {
        *self.balances.get(addr).unwrap_or(&0)
    }

    pub fn apply(&mut self, tx: &Transaction) {
        let from_balance = self.get_balance(&tx.from);
        let to_balance = self.get_balance(&tx.to);

        // debit
        self.balances.insert(tx.from.clone(), from_balance.saturating_sub(tx.amount));

        // credit
        self.balances.insert(tx.to.clone(), to_balance.saturating_add(tx.amount));

        // update nonce
        self.nonces.insert(tx.from.clone(), tx.nonce);
    }

    pub fn get_nonce(&self, addr: &str) -> u64 {
        *self.nonces.get(addr).unwrap_or(&0)
    }
}
