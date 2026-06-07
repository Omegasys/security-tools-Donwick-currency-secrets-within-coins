use std::collections::HashMap;

pub fn run(address: Option<String>) {
    let addr = address.unwrap_or_else(|| "default-wallet".to_string());

    // In real system this comes from ledger state
    let mut balances = HashMap::new();
    balances.insert(addr.clone(), 1000u64);

    println!("Wallet: {}", addr);
    println!("Balance: {} AXM", balances.get(&addr).unwrap());
}
