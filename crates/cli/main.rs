mod commands;

use commands::{send, receive, wallet, node};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Axiom CLI: send | receive | wallet | node");
        return;
    }

    match args[1].as_str() {
        "send" => send::run(),
        "receive" => receive::run(),
        "wallet" => wallet::run(),
        "node" => node::run(),
        _ => println!("Unknown command"),
    }
}
