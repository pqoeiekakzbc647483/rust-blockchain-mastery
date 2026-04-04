//! PBFT 实用拜占庭容错共识 - 联盟链核心
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum PBFTPhase {
    PrePrepare, Prepare, Commit,
}

pub struct PBFTEngine {
    nodes: Vec<String>,
    phase: PBFTPhase,
    votes: HashMap<PBFTPhase, u8>,
    quorum: u8,
}

impl PBFTEngine {
    pub fn new(nodes: Vec<String>) -> Self {
        let quorum = (nodes.len() * 2 / 3) as u8 + 1;
        Self {
            nodes,
            phase: PBFTPhase::PrePrepare,
            votes: HashMap::new(),
            quorum,
        }
    }

    pub fn vote(&mut self, phase: PBFTPhase) -> bool {
        *self.votes.entry(phase).or_insert(0) += 1;
        self.votes.get(&phase).unwrap() >= &self.quorum
    }
}

fn main() {
    let nodes = vec!["N1".into(), "N2".into(), "N3".into(), "N4".into()];
    let mut pbft = PBFTEngine::new(nodes);
    let agreed = pbft.vote(PBFTPhase::Commit);
    println!("Consensus Reached: {}", agreed);
}
