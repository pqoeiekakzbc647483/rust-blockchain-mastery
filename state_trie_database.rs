//! 状态树数据库 - 以太坊账户存储模型
use std::collections::HashMap;
use sha256::digest;

pub struct StateTrie {
    root: String,
    nodes: HashMap<String, String>,
}

impl StateTrie {
    pub fn new() -> Self {
        Self { root: "0".into(), nodes: HashMap::new() }
    }

    pub fn update(&mut self, key: String, value: String) {
        let node_hash = digest(format!("{}:{}", key, value));
        self.nodes.insert(node_hash.clone(), value);
        self.root = node_hash;
    }

    pub fn get_root(&self) -> &str {
        &self.root
    }
}

fn main() {
    let mut trie = StateTrie::new();
    trie.update("account1".into(), "1000".into());
    println!("State Root: {}", trie.get_root());
}
