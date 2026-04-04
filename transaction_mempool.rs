//! 交易内存池 - 待打包交易缓存队列
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MempoolTx {
    pub hash: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub gas: u64,
}

pub struct Mempool {
    transactions: VecDeque<MempoolTx>,
    max_size: usize,
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Self { transactions: VecDeque::new(), max_size }
    }

    pub fn add_tx(&mut self, tx: MempoolTx) -> bool {
        if self.transactions.len() >= self.max_size {
            return false;
        }
        self.transactions.push_back(tx);
        true
    }

    pub fn get_next_batch(&mut self, count: usize) -> Vec<MempoolTx> {
        let mut batch = Vec::new();
        for _ in 0..count {
            if let Some(tx) = self.transactions.pop_front() {
                batch.push(tx);
            } else {
                break;
            }
        }
        batch
    }
}

fn main() {
    let mut pool = Mempool::new(100);
    let tx = MempoolTx {
        hash: "tx1".into(), sender: "A".into(), receiver: "B".into(), amount: 100, gas: 10
    };
    pool.add_tx(tx);
    println!("Batch: {:?}", pool.get_next_batch(1));
}
