/// Node configuration settings
#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub listen_addr: String,
    pub max_peers: usize,
    pub node_name: String,
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            listen_addr: "127.0.0.1:3030".to_string(),
            max_peers: 50,
            node_name: "axiom-node".to_string(),
        }
    }
}
