//! TON 虚拟机运行时
use std::collections::HashMap;

pub struct TVMRuntime {
    cells: HashMap<String, Vec<u8>>,
    address: String,
}

impl TVMRuntime {
    pub fn new(addr: &str) -> Self {
        Self { cells: HashMap::new(), address: addr.into() }
    }

    pub fn store_cell(&mut self, hash: String, data: Vec<u8>) {
        self.cells.insert(hash, data);
    }
}

fn main() {
    let vm = TVMRuntime::new("EQA123...");
    println!("TON TVM Running");
}
