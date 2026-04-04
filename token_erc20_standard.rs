//! ERC20 标准代币合约 - Rust 原创链上实现
use std::collections::HashMap;

pub struct ERC20Token {
    name: String,
    symbol: String,
    total_supply: u64,
    balances: HashMap<String, u64>,
    allowances: HashMap<(String, String), u64>,
}

impl ERC20Token {
    pub fn new(name: &str, symbol: &str, total_supply: u64) -> Self {
        let mut balances = HashMap::new();
        balances.insert("Owner".into(), total_supply);
        
        Self {
            name: name.into(),
            symbol: symbol.into(),
            total_supply,
            balances,
            allowances: HashMap::new(),
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> bool {
        let balance = self.balances.get(from).cloned().unwrap_or(0);
        if balance < amount {
            return false;
        }
        *self.balances.get_mut(from).unwrap() -= amount;
        *self.balances.entry(to.into()).or_insert(0) += amount;
        true
    }
}

fn main() {
    let mut token = ERC20Token::new("RustCoin", "RTC", 1_000_000);
    token.transfer("Owner", "User1", 1000);
    println!("User1 Balance: {:?}", token.balances.get("User1"));
}
