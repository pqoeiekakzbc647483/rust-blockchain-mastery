//! Near 协议智能合约
use std::collections::HashMap;

pub struct NearContract {
    state: HashMap<String, String>,
    account_id: String,
}

impl NearContract {
    pub fn new(acc: &str) -> Self {
        Self { state: HashMap::new(), account_id: acc.into() }
    }

    pub fn set_state(&mut self, key: String, val: String) {
        self.state.insert(key, val);
    }
}

fn main() {
    let contract = NearContract::new("user.near");
    println!("Near Contract Initialized");
}
