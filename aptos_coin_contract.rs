//! Aptos 标准化代币合约
use std::collections::HashMap;

pub struct AptosCoin {
    symbol: String,
    supply: u64,
    balances: HashMap<String, u64>,
}

impl AptosCoin {
    pub fn new(symbol: &str, supply: u64) -> Self {
        Self { symbol: symbol.into(), supply, balances: HashMap::new() }
    }

    pub fn mint(&mut self, to: String, amount: u64) {
        *self.balances.entry(to).or_insert(0) += amount;
        self.supply += amount;
    }
}

fn main() {
    let coin = AptosCoin::new("APT", 1_000_000_000);
    println!("Aptos Coin Deployed");
}
