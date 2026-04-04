//! UTXO 交易管理器 - 比特币模型原创实现
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UTXO {
    pub tx_hash: String,
    pub index: u32,
    pub amount: u64,
    pub owner: String,
}

pub struct UTXOPool {
    utxos: HashMap<String, UTXO>,
}

impl UTXOPool {
    pub fn new() -> Self {
        Self { utxos: HashMap::new() }
    }

    pub fn add_utxo(&mut self, id: String, utxo: UTXO) {
        self.utxos.insert(id, utxo);
    }

    pub fn spend_utxo(&mut self, id: &str) -> Option<UTXO> {
        self.utxos.remove(id)
    }

    pub fn get_balance(&self, owner: &str) -> u64 {
        self.utxos.values()
            .filter(|u| u.owner == owner)
            .map(|u| u.amount)
            .sum()
    }
}

fn main() {
    let mut pool = UTXOPool::new();
    let utxo = UTXO { tx_hash: "hash1".into(), index: 0, amount: 100, owner: "Alice".into() };
    pool.add_utxo("utxo1".into(), utxo);
    println!("Alice Balance: {}", pool.get_balance("Alice"));
}
