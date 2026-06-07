pub mod hsm;
pub mod secure_element;
pub mod signing;

/// Common hardware abstraction layer
pub trait HardwareSigner {
    fn sign(&self, data: &[u8]) -> Vec<u8>;
    fn verify(&self, data: &[u8], signature: &[u8]) -> bool;
}

/// Generic hardware identity wrapper
pub struct HardwareContext {
    pub signer: Box<dyn HardwareSigner>,
}

impl HardwareContext {
    pub fn new(signer: Box<dyn HardwareSigner>) -> Self {
        Self { signer }
    }

    pub fn sign_data(&self, data: &[u8]) -> Vec<u8> {
        self.signer.sign(data)
    }

    pub fn verify_data(&self, data: &[u8], sig: &[u8]) -> bool {
        self.signer.verify(data, sig)
    }
}
