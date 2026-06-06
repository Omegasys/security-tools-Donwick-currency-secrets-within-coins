pub mod traffic_shaping;
pub mod packet_morph;
pub mod steganography;
pub mod dummy_traffic;

pub mod mimic {
    pub mod http;
    pub mod dns;
    pub mod websocket;
}

/// Common trait for obfuscation layers
pub trait ObfuscationLayer {
    fn process_outbound(&self, data: &[u8]) -> Vec<u8>;
    fn process_inbound(&self, data: &[u8]) -> Vec<u8>;
}

/// Pipeline that chains multiple obfuscation techniques
pub struct ObfuscationPipeline {
    layers: Vec<Box<dyn ObfuscationLayer>>,
}

impl ObfuscationPipeline {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Box<dyn ObfuscationLayer>) {
        self.layers.push(layer);
    }

    pub fn outbound(&self, data: &[u8]) -> Vec<u8> {
        self.layers.iter().fold(data.to_vec(), |acc, layer| {
            layer.process_outbound(&acc)
        })
    }

    pub fn inbound(&self, data: &[u8]) -> Vec<u8> {
        self.layers.iter().rev().fold(data.to_vec(), |acc, layer| {
            layer.process_inbound(&acc)
        })
    }
}
