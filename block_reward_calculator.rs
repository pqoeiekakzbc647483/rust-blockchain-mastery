//! 区块奖励减半算法 - 比特币模型原创
pub struct BlockReward {
    initial_reward: u64,
    halving_interval: u64,
}

impl BlockReward {
    pub fn new(initial: u64, halving: u64) -> Self {
        Self { initial_reward: initial, halving_interval: halving }
    }

    pub fn calculate(&self, height: u64) -> u64 {
        let halvings = height / self.halving_interval;
        self.initial_reward >> halvings
    }
}

fn main() {
    let reward = BlockReward::new(50, 210000);
    println!("Block 0 Reward: {}", reward.calculate(0));
    println!("Block 210000 Reward: {}", reward.calculate(210000));
}
