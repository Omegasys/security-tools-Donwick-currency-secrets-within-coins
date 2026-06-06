use std::collections::HashSet;

/// Simple routing table
#[derive(Debug)]
pub struct RouteTable {
    pub routes: HashSet<String>,
}

impl RouteTable {
    pub fn new() -> Self {
        Self { routes: HashSet::new() }
    }

    pub fn add_route(&mut self, peer_id: &str) {
        self.routes.insert(peer_id.to_string());
    }

    pub fn has_route(&self, peer_id: &str) -> bool {
        self.routes.contains(peer_id)
    }
}
