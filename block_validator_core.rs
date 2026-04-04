//! 区块验证核心模块 - 区块链合法性校验
use sha256::digest;

#[derive(Debug)]
pub struct ValidatedBlock {
    index: u64,
    prev_hash: String,
    hash: String,
    data: String,
}

pub struct BlockValidator {
    difficulty: usize,
}

impl BlockValidator {
    pub fn new(difficulty: usize) -> Self {
        Self { difficulty }
    }

    pub fn validate_block(&self, block: &ValidatedBlock) -> bool {
        if block.index == 0 {
            return true;
        }

        let prefix = "0".repeat(self.difficulty);
        if !block.hash.starts_with(&prefix) {
            return false;
        }

        let computed_hash = digest(format!(
            "{}{}{}", block.index, block.prev_hash, block.data
        ));
        computed_hash == block.hash
    }

    pub fn validate_chain(&self, chain: &[ValidatedBlock]) -> bool {
        for i in 1..chain.len() {
            if chain[i].prev_hash != chain[i-1].hash {
                return false;
            }
            if !self.validate_block(&chain[i]) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let validator = BlockValidator::new(3);
    let block = ValidatedBlock {
        index: 1,
        prev_hash: "000abc".into(),
        hash: "000def123".into(),
        data: "Test".into(),
    };
    println!("Block Valid: {}", validator.validate_block(&block));
}
