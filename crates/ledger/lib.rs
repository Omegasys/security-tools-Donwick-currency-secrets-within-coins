pub mod transaction;
pub mod block;
pub mod state;
pub mod validation;

use transaction::Transaction;
use block::Block;
use state::LedgerState;

/// Core ledger interface
pub struct Ledger {
    pub state: LedgerState,
    pub chain: Vec<Block>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            state: LedgerState::new(),
            chain: Vec::new(),
        }
    }

    /// Apply a validated transaction into state
    pub fn apply_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        validation::validate_transaction(&tx, &self.state)?;

        self.state.apply(&tx);
        Ok(())
    }

    /// Add a block after validation
    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
        validation::validate_block(&block, &self.state)?;

        for tx in &block.transactions {
            self.state.apply(tx);
        }

        self.chain.push(block);
        Ok(())
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }
}
