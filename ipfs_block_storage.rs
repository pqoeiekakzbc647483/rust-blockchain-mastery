//! IPFS 区块分布式存储 - 原创适配层
use sha256::digest;
use std::collections::HashMap;

pub struct IPFSStorage {
    store: HashMap<String, Vec<u8>>,
}

impl IPFSStorage {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    pub fn store_block(&mut self, data: Vec<u8>) -> String {
        let cid = digest(&data);
        self.store.insert(cid.clone(), data);
        cid
    }

    pub fn retrieve_block(&self, cid: &str) -> Option<&Vec<u8>> {
        self.store.get(cid)
    }

    pub fn delete_block(&mut self, cid: &str) -> bool {
        self.store.remove(cid).is_some()
    }
}

fn main() {
    let mut storage = IPFSStorage::new();
    let data = b"Blockchain Block Data".to_vec();
    let cid = storage.store_block(data);
    println!("Stored CID: {}", cid);
    println!("Retrieved: {:?}", storage.retrieve_block(&cid));
}
