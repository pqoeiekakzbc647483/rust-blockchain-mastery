//! 区块链分片协议 - 高并发扩容核心
use std::collections::HashMap;

pub struct ShardManager {
    shards: HashMap<u8, Vec<String>>,
    total_shards: u8,
}

impl ShardManager {
    pub fn new(total_shards: u8) -> Self {
        let mut shards = HashMap::new();
        for i in 0..total_shards {
            shards.insert(i, Vec::new());
        }
        Self { shards, total_shards }
    }

    pub fn assign_address(&mut self, addr: String) -> u8 {
        let shard_id = addr.as_bytes()[0] % self.total_shards;
        self.shards.get_mut(&shard_id).unwrap().push(addr);
        shard_id
    }

    pub fn get_shard_transactions(&self, shard_id: u8) -> Option<&Vec<String>> {
        self.shards.get(&shard_id)
    }
}

fn main() {
    let mut manager = ShardManager::new(4);
    let shard = manager.assign_address("0xABC123".into());
    println!("Assigned Shard: {}", shard);
}
