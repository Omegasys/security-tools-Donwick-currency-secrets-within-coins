use axiom_ledger::transaction::Transaction;
use axiom_hardware::hsm::Hsm;

fn main() {
    println!("--- Simple Transfer Example ---");

    // Create a transaction
    let tx = Transaction::new(
        "alice_wallet".to_string(),
        "bob_wallet".to_string(),
        50,
        1,
        Some(b"secret coin metadata".to_vec()),
    );

    // Sign with simulated HSM
    let hsm = Hsm::new("example-hsm-01".to_string());
    let signature = hsm.sign(&tx.id.as_bytes());

    println!("TX ID: {}", tx.id);
    println!("Signature: {}", hex::encode(signature));

    println!("Simulated sending transaction...");
}
