//! 区块链核心账本底层框架 - 原创实现
use sha256::{digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, prev_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let nonce = 0;
        let hash = Self::calculate_hash(index, timestamp, &data, &prev_hash, nonce);
        
        Self { index, timestamp, data, prev_hash, hash, nonce }
    }

    fn calculate_hash(index: u64, ts: u128, data: &str, prev: &str, nonce: u64) -> String {
        let input = format!("{}{}{}{}{}", index, ts, data, prev, nonce);
        digest(input)
    }
}

pub struct BlockchainLedger {
    pub chain: Vec<Block>,
}

impl BlockchainLedger {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        chain.push(Block::new(0, "Genesis Block".to_string(), "0".to_string()));
        Self { chain }
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(prev_block.index + 1, data, prev_block.hash.clone());
        self.chain.push(new_block);
    }
}

fn main() {
    let mut ledger = BlockchainLedger::new();
    ledger.add_block("Transaction: Alice -> Bob 100 Tokens".to_string());
    ledger.add_block("Transaction: Bob -> Charlie 50 Tokens".to_string());
    println!("{:?}", ledger.chain);
}
