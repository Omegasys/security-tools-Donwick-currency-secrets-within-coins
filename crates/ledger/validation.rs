use crate::ledger::transaction::Transaction;
use crate::ledger::block::Block;
use crate::ledger::state::LedgerState;

/// Validate a single transaction
pub fn validate_transaction(tx: &Transaction, state: &LedgerState) -> Result<(), String> {
    if tx.amount == 0 {
        return Err("Transaction amount cannot be zero".into());
    }

    let sender_balance = state.get_balance(&tx.from);
    if sender_balance < tx.amount {
        return Err("Insufficient balance".into());
    }

    let expected_nonce = state.get_nonce(&tx.from);
    if tx.nonce != expected_nonce + 1 {
        return Err("Invalid nonce".into());
    }

    Ok(())
}

/// Validate a block
pub fn validate_block(block: &Block, state: &LedgerState) -> Result<(), String> {
    if block.transactions.is_empty() {
        return Err("Block has no transactions".into());
    }

    // Basic structural check
    if block.hash != block.compute_hash() {
        return Err("Invalid block hash".into());
    }

    // Validate all transactions
    let mut temp_state = state.clone();

    for tx in &block.transactions {
        validate_transaction(tx, &temp_state)?;
        temp_state.apply(tx);
    }

    Ok(())
}
