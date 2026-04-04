//! P2P 加密隧道 - 节点通信安全加密
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

pub struct EncryptedTunnel {
    cipher: Aes256Gcm,
}

impl EncryptedTunnel {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let nonce = Nonce::from_slice(b"unique_nonce_123");
        self.cipher.encrypt(nonce, data).unwrap()
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let nonce = Nonce::from_slice(b"unique_nonce_123");
        self.cipher.decrypt(nonce, data).unwrap()
    }
}

fn main() {
    let key = [0u8; 32];
    let tunnel = EncryptedTunnel::new(&key);
    let encrypted = tunnel.encrypt(b"Secret Message");
    let decrypted = tunnel.decrypt(&encrypted);
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
}
