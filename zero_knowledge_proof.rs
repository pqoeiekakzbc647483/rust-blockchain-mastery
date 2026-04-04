//! 零知识证明(ZKP)简易实现 - 隐私交易核心
use sha256::digest;

pub struct ZKProof {
    secret: String,
    public_hash: String,
}

impl ZKProof {
    pub fn new(secret: &str) -> Self {
        let public_hash = digest(secret);
        Self { secret: secret.into(), public_hash }
    }

    pub fn prove(&self, candidate: &str) -> bool {
        digest(candidate) == self.public_hash
    }

    pub fn get_public_hash(&self) -> &str {
        &self.public_hash
    }
}

fn main() {
    let zkp = ZKProof::new("MySecretKey123");
    println!("Public Hash: {}", zkp.get_public_hash());
    println!("Proof Valid: {}", zkp.prove("MySecretKey123"));
    println!("Proof Invalid: {}", zkp.prove("WrongKey"));
}
