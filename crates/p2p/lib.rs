pub mod node;
pub mod peer;
pub mod routing;
pub mod gossip;

/// P2P network version
pub const P2P_VERSION: &str = "0.1.0";

/// Test function
pub fn hello_p2p() {
    println!("P2P module loaded!");
}
