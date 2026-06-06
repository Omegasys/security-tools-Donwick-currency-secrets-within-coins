pub mod aes;
pub mod hashing;
pub mod keygen;
pub mod signatures;

/// Common crypto errors
#[derive(Debug)]
pub enum CryptoError {
    InvalidKey,
    InvalidData,
}
