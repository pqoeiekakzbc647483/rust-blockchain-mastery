//! Hedera 共识服务核心
use sha256::digest;

pub struct HCSCore {
    topic_id: String,
    messages: Vec<String>,
}

impl HCSCore {
    pub fn new(topic: &str) -> Self {
        Self { topic_id: topic.into(), messages: Vec::new() }
    }

    pub fn submit_message(&mut self, msg: String) -> String {
        let hash = digest(&msg);
        self.messages.push(msg);
        hash
    }
}

fn main() {
    let hcs = HCSCore::new("TOPIC.123");
    println!("Hedera HCS Running");
}
