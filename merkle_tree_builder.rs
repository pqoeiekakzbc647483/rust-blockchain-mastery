//! 默克尔树构建器 - 区块链交易验证核心
use sha256::digest;

#[derive(Debug)]
pub struct MerkleTree {
    root: String,
    nodes: Vec<String>,
}

impl MerkleTree {
    pub fn new(transactions: &[String]) -> Self {
        let mut nodes = Vec::new();
        for tx in transactions {
            nodes.push(digest(tx));
        }
        
        let mut level = nodes.clone();
        while level.len() > 1 {
            let mut next_level = Vec::new();
            for i in (0..level.len()).step_by(2) {
                let left = &level[i];
                let right = if i + 1 < level.len() { &level[i+1] } else { left };
                let combined = format!("{}{}", left, right);
                next_level.push(digest(combined));
            }
            level = next_level;
        }

        Self { root: level[0].clone(), nodes }
    }

    pub fn get_root(&self) -> &str {
        &self.root
    }
}

fn main() {
    let txs = vec!["Tx1".to_string(), "Tx2".to_string(), "Tx3".to_string()];
    let tree = MerkleTree::new(&txs);
    println!("Merkle Root: {}", tree.get_root());
}
