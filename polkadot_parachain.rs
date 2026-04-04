//! Polkadot 平行链核心 - 中继链互联
use std::collections::HashMap;

pub struct Parachain {
    para_id: u32,
    collators: Vec<String>,
    relay_chain: String,
}

impl Parachain {
    pub fn new(id: u32, relay: &str) -> Self {
        Self {
            para_id: id,
            collators: Vec::new(),
            relay_chain: relay.into(),
        }
    }

    pub fn add_collator(&mut self, addr: String) {
        self.collators.push(addr);
    }
}

fn main() {
    let para = Parachain::new(100, "POLKADOT-RELAY");
    println!("Parachain ID: {}", para.para_id);
}
