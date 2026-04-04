//! 区块链钱包核心 - 密钥/地址/交易管理
use sha256::digest;
use rand::Rng;

pub struct Wallet {
    private_key: String,
    public_key: String,
    address: String,
    balance: u64,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let private_key: String = (0..32).map(|_| rng.gen_range(0..9).to_string()).collect();
        let public_key = digest(&private_key);
        let address = format!("0x{}", &public_key[0..40]);
        
        Self { private_key, public_key, address, balance: 0 }
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn add_balance(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn send_transaction(&mut self, amount: u64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
}

fn main() {
    let wallet = Wallet::new();
    println!("Wallet Address: {}", wallet.get_address());
}
