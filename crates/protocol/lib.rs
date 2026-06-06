pub mod message;
pub mod encoding;
pub mod handshake;

/// Protocol version
pub const PROTOCOL_VERSION: &str = "0.1.0";

/// Simple test function
pub fn hello_protocol() {
    println!("Protocol module loaded!");
}
