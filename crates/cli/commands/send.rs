use axiom_ledger::transaction::Transaction;
use axiom_hardware::hsm::Hsm;
use axiom_crypto::hashing::hash_bytes;

pub fn run(from: String, to: String, amount: u64) {
    println!("Creating transaction...");

    let tx = Transaction::new(from.clone(), to.clone(), amount, 1, None);

    // simulate hardware signing
    let hsm = Hsm::new("axiom-hsm-01".to_string());
    let signature = hsm.sign(tx.id.as_bytes());

    println!("TX ID: {}", tx.id);
    println!("Signed with HSM signature: {}", hex::encode(signature));

    println!("Broadcasting transaction (mock P2P)...");
}
