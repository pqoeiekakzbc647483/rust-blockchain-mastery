//! ECDSA 区块链签名加密模块 - 原创实现
use k256::{ecdsa::{SigningKey, VerifyingKey, signature::Signer, signature::Verifier}, Secp256k1};
use rand::rngs::OsRng;

pub struct CryptoSigner {
    secret_key: SigningKey,
    public_key: VerifyingKey,
}

impl CryptoSigner {
    pub fn new() -> Self {
        let secret_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&secret_key);
        Self { secret_key, public_key }
    }

    pub fn sign_message(&self, msg: &[u8]) -> Vec<u8> {
        let signature = self.secret_key.sign(msg);
        signature.to_vec()
    }

    pub fn verify_message(&self, msg: &[u8], sig: &[u8]) -> bool {
        let signature = k256::ecdsa::Signature::from_slice(sig).unwrap();
        self.public_key.verify(msg, &signature).is_ok()
    }
}

fn main() {
    let signer = CryptoSigner::new();
    let msg = b"Blockchain Transaction Signature";
    let sig = signer.sign_message(msg);
    let is_valid = signer.verify_message(msg, &sig);
    
    println!("Signature Valid: {}", is_valid);
}
