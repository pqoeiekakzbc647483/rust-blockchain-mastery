//! 多签钱包 - N/M 授权转账
use std::collections::HashMap;

pub struct MultiSigWallet {
    owners: Vec<String>,
    required: u8,
    transactions: HashMap<u64, MultiSigTx>,
    next_id: u64,
}

#[derive(Debug)]
struct MultiSigTx {
    to: String,
    amount: u64,
    confirmations: Vec<String>,
    executed: bool,
}

impl MultiSigWallet {
    pub fn new(owners: Vec<String>, required: u8) -> Self {
        Self { owners, required, transactions: HashMap::new(), next_id: 1 }
    }

    pub fn submit_tx(&mut self, to: String, amount: u64) -> u64 {
        let id = self.next_id;
        self.transactions.insert(id, MultiSigTx {
            to, amount, confirmations: Vec::new(), executed: false
        });
        self.next_id += 1;
        id
    }
}

fn main() {
    let owners = vec!["A".into(), "B".into(), "C".into()];
    let wallet = MultiSigWallet::new(owners, 2);
    println!("MultiSig Wallet Initialized");
}
