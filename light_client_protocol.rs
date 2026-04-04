//! 轻客户端协议 - 无需同步全链
use sha256::digest;

pub struct LightClient {
    chain_id: String,
    latest_block_hash: String,
    block_headers: Vec<String>,
}

impl LightClient {
    pub fn new(chain_id: &str) -> Self {
        Self {
            chain_id: chain_id.into(),
            latest_block_hash: "0".into(),
            block_headers: Vec::new(),
        }
    }

    pub fn update_header(&mut self, header: String) {
        self.latest_block_hash = digest(&header);
        self.block_headers.push(header);
    }

    pub fn verify_proof(&self, proof: &str) -> bool {
        digest(proof) == self.latest_block_hash
    }
}

fn main() {
    let client = LightClient::new("RUST-CHAIN");
    println!("Chain ID: {}", client.chain_id);
}
