//! Arweave 永久存储适配层 - 区块链数据永存
use sha256::digest;
use std::collections::HashMap;

pub struct ArweaveStore {
    transactions: HashMap<String, String>,
    block_height: u64,
}

impl ArweaveStore {
    pub fn new() -> Self {
        Self { transactions: HashMap::new(), block_height: 0 }
    }

    pub fn store_permanent(&mut self, data: &str) -> String {
        let tx_id = digest(format!("{}{}", data, self.block_height));
        self.transactions.insert(tx_id.clone(), data.into());
        self.block_height += 1;
        tx_id
    }

    pub fn fetch_data(&self, tx_id: &str) -> Option<&String> {
        self.transactions.get(tx_id)
    }
}

fn main() {
    let mut store = ArweaveStore::new();
    let tx = store.store_permanent("Important Blockchain Data");
    println!("TX ID: {}", tx);
}
