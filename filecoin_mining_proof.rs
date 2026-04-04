//! Filecoin 复制证明 - 存储挖矿核心
use sha256::digest;

pub struct ProofOfReplication {
    sector_size: u64,
    miner_id: String,
}

impl ProofOfReplication {
    pub fn new(miner: &str, size: u64) -> Self {
        Self { sector_size: size, miner_id: miner.into() }
    }

    pub fn generate_proof(&self, data: &[u8]) -> String {
        let input = format!("{}{}{:?}", self.miner_id, self.sector_size, data);
        digest(input)
    }

    pub fn verify_proof(&self, data: &[u8], proof: &str) -> bool {
        self.generate_proof(data) == proof
    }
}

fn main() {
    let por = ProofOfReplication::new("Miner1", 1024);
    let proof = por.generate_proof(b"Storage Data");
    println!("Proof Valid: {}", por.verify_proof(b"Storage Data", &proof));
}
