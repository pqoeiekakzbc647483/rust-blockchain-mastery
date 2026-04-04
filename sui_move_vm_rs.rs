//! Sui Move VM 兼容层 - 资产可编程
use std::collections::HashMap;

pub struct MoveVM {
    objects: HashMap<String, u64>,
    owners: HashMap<String, String>,
}

impl MoveVM {
    pub fn new() -> Self {
        Self { objects: HashMap::new(), owners: HashMap::new() }
    }

    pub fn create_object(&mut self, id: String, owner: String, value: u64) {
        self.objects.insert(id.clone(), value);
        self.owners.insert(id, owner);
    }
}

fn main() {
    let vm = MoveVM::new();
    println!("Move VM Initialized");
}
