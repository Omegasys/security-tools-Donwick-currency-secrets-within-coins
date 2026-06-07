#[cfg(test)]
mod tests {
    use axiom_ledger::ledger::Ledger;
    use axiom_ledger::transaction::Transaction;

    #[test]
    fn test_simple_transaction_flow() {
        let mut ledger = Ledger::new();

        let tx = Transaction::new(
            "alice".to_string(),
            "bob".to_string(),
            100,
            1,
            Some(b"coin data".to_vec()),
        );

        // apply transaction
        ledger.apply_transaction(tx.clone()).unwrap();

        assert_eq!(ledger.state.get_balance("alice"), 0);
        assert_eq!(ledger.state.get_balance("bob"), 100);
    }
}
