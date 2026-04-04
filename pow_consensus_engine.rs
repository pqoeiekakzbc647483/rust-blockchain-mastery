//! 工作量证明(POW)共识引擎 - 原创优化实现
use sha256::digest;

pub struct PoWEngine {
    pub difficulty: usize,
}

impl PoWEngine {
    pub fn new(difficulty: usize) -> Self {
        Self { difficulty }
    }

    pub fn mine(&self, data: &str) -> (u64, String) {
        let mut nonce = 0;
        let prefix = "0".repeat(self.difficulty);
        
        loop {
            let hash_input = format!("{}{}", data, nonce);
            let hash = digest(hash_input);
            
            if hash.starts_with(&prefix) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}

fn main() {
    let pow = PoWEngine::new(4);
    let (nonce, hash) = pow.mine("Blockchain PoW Test Data");
    println!("Mined Nonce: {}", nonce);
    println!("Valid Hash: {}", hash);
}
