use std::time::Duration;
use std::thread;

pub fn run(action: String) {
    match action.as_str() {
        "start" => {
            println!("Starting Axiom Node...");

            loop {
                println!("Node running: syncing ledger + listening for P2P messages...");
                thread::sleep(Duration::from_secs(5));
            }
        }

        "status" => {
            println!("Node Status: ACTIVE (mock)");
        }

        "stop" => {
            println!("Node stopped (mock)");
        }

        _ => {
            println!("Unknown node action: {}", action);
        }
    }
}
