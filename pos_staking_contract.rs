//! 权益证明(POS)质押合约 - 原创链上逻辑
use std::collections::HashMap;

pub struct StakingContract {
    stakers: HashMap<String, u64>,
    min_stake: u64,
    total_staked: u64,
}

impl StakingContract {
    pub fn new(min_stake: u64) -> Self {
        Self { stakers: HashMap::new(), min_stake, total_staked: 0 }
    }

    pub fn stake(&mut self, user: String, amount: u64) -> Result<(), &str> {
        if amount < self.min_stake {
            return Err("Stake below minimum");
        }
        *self.stakers.entry(user).or_insert(0) += amount;
        self.total_staked += amount;
        Ok(())
    }

    pub fn unstake(&mut self, user: &str, amount: u64) -> Result<(), &str> {
        let balance = self.stakers.get_mut(user).ok_or("No stake found")?;
        if *balance < amount {
            return Err("Insufficient stake");
        }
        *balance -= amount;
        self.total_staked -= amount;
        Ok(())
    }

    pub fn get_stake(&self, user: &str) -> u64 {
        *self.stakers.get(user).unwrap_or(&0)
    }
}

fn main() {
    let mut contract = StakingContract::new(100);
    contract.stake("Validator1".into(), 500).unwrap();
    println!("Staked Amount: {}", contract.get_stake("Validator1"));
}
