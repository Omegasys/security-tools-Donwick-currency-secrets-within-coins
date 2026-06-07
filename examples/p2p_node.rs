use std::thread;
use std::time::Duration;

fn main() {
    println!("--- P2P Node Example ---");

    println!("Starting simulated node...");
    loop {
        println!("Node running: listening for peers...");
        // Placeholder: would hook into your p2p crate
        thread::sleep(Duration::from_secs(3));
    }
}
