//! 委托权益证明(DPOS)投票系统 - 原创实现
use std::collections::HashMap;

pub struct DPoSVoting {
    candidates: HashMap<String, u64>,
    voters: HashMap<String, String>,
}

impl DPoSVoting {
    pub fn new() -> Self {
        Self { candidates: HashMap::new(), voters: HashMap::new() }
    }

    pub fn register_candidate(&mut self, addr: String) {
        self.candidates.insert(addr, 0);
    }

    pub fn vote(&mut self, voter: String, candidate: String) -> Result<(), &str> {
        if !self.candidates.contains_key(&candidate) {
            return Err("Candidate not found");
        }
        if self.voters.contains_key(&voter) {
            return Err("Already voted");
        }
        self.voters.insert(voter, candidate.clone());
        *self.candidates.get_mut(&candidate).unwrap() += 1;
        Ok(())
    }

    pub fn get_top_candidates(&self, count: usize) -> Vec<(String, u64)> {
        let mut list: Vec<_> = self.candidates.iter().map(|(k,v)| (k.clone(), *v)).collect();
        list.sort_by(|a,b| b.1.cmp(&a.1));
        list.into_iter().take(count).collect()
    }
}

fn main() {
    let mut voting = DPoSVoting::new();
    voting.register_candidate("Node1".into());
    voting.vote("User1".into(), "Node1".into()).unwrap();
    println!("Top Nodes: {:?}", voting.get_top_candidates(1));
}
