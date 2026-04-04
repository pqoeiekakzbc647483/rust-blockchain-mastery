//! 跨链桥核心协议 - 原创异构链交互
use std::collections::VecDeque;

#[derive(Debug)]
pub struct CrossChainTx {
    pub source_chain: String,
    pub target_chain: String,
    pub amount: u64,
    pub sender: String,
    pub receiver: String,
}

pub struct CrossChainBridge {
    tx_queue: VecDeque<CrossChainTx>,
    supported_chains: Vec<String>,
}

impl CrossChainBridge {
    pub fn new() -> Self {
        Self {
            tx_queue: VecDeque::new(),
            supported_chains: vec!["ETH".into(), "SOL".into(), "BTC".into()],
        }
    }

    pub fn add_transaction(&mut self, tx: CrossChainTx) -> Result<(), &str> {
        if !self.supported_chains.contains(&tx.source_chain) || 
           !self.supported_chains.contains(&tx.target_chain) {
            return Err("Unsupported chain");
        }
        self.tx_queue.push_back(tx);
        Ok(())
    }

    pub fn process_next(&mut self) -> Option<CrossChainTx> {
        self.tx_queue.pop_front()
    }
}

fn main() {
    let mut bridge = CrossChainBridge::new();
    let tx = CrossChainTx {
        source_chain: "ETH".into(),
        target_chain: "SOL".into(),
        amount: 1000,
        sender: "0x123".into(),
        receiver: "SoL456".into(),
    };
    bridge.add_transaction(tx).unwrap();
    println!("Processing: {:?}", bridge.process_next());
}
