//! Stellar Soroban 智能合约
use std::collections::HashMap;

pub struct SorobanContract {
    storage: HashMap<String, String>,
    contract_id: String,
}

impl SorobanContract {
    pub fn new(id: &str) -> Self {
        Self { storage: HashMap::new(), contract_id: id.into() }
    }

    pub fn put(&mut self, key: String, val: String) {
        self.storage.insert(key, val);
    }
}

fn main() {
    let contract = SorobanContract::new("SOROBAN123");
    println!("Soroban Contract Ready");
}
