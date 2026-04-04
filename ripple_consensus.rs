//! Ripple 共识协议
use std::collections::HashMap;

pub struct RippleConsensus {
    nodes: Vec<String>,
    ledger_hash: String,
}

impl RippleConsensus {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), ledger_hash: "0".into() }
    }

    pub fn update_ledger(&mut self, hash: String) {
        self.ledger_hash = hash;
    }
}

fn main() {
    let consensus = RippleConsensus::new();
    println!("Ripple Consensus Ready");
}
