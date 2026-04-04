//! Avalanche 雪崩共识 - 高速亚秒级共识
use std::collections::HashMap;

pub struct Avalanche {
    validators: Vec<String>,
    confidence: HashMap<String, u8>,
    threshold: u8,
}

impl Avalanche {
    pub fn new(validators: Vec<String>, threshold: u8) -> Self {
        Self { validators, confidence: HashMap::new(), threshold }
    }

    pub fn sample_vote(&mut self, tx: &str, approve: bool) -> bool {
        let cnt = self.confidence.entry(tx.into()).or_insert(0);
        if approve { *cnt += 1 } else { *cnt -= 1 }
        *cnt >= self.threshold
    }
}

fn main() {
    let validators = vec!["V1".into(), "V2".into(), "V3".into()];
    let mut avax = Avalanche::new(validators, 2);
    let accepted = avax.sample_vote("tx1", true);
    println!("Accepted: {}", accepted);
}
