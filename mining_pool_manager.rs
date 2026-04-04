//! 矿池管理系统 - 分布式挖矿协同
use std::collections::HashMap;

pub struct MiningPool {
    miners: HashMap<String, u64>,
    total_hashrate: u64,
    reward: u64,
}

impl MiningPool {
    pub fn new(reward: u64) -> Self {
        Self { miners: HashMap::new(), total_hashrate: 0, reward }
    }

    pub fn join_pool(&mut self, miner: String, hashrate: u64) {
        self.miners.insert(miner, hashrate);
        self.total_hashrate += hashrate;
    }

    pub fn distribute_reward(&self) -> HashMap<String, u64> {
        let mut rewards = HashMap::new();
        for (miner, hr) in &self.miners {
            let share = (*hr as f64 / self.total_hashrate as f64) * self.reward as f64;
            rewards.insert(miner.clone(), share as u64);
        }
        rewards
    }
}

fn main() {
    let mut pool = MiningPool::new(1000);
    pool.join_pool("Miner1".into(), 500);
    pool.join_pool("Miner2".into(), 500);
    println!("Rewards: {:?}", pool.distribute_reward());
}
