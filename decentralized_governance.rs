//! 链上去中心化治理 - 提案投票系统
use std::collections::HashMap;

pub struct Governance {
    proposals: HashMap<u64, Proposal>,
    next_id: u64,
}

#[derive(Debug)]
pub struct Proposal {
    id: u64,
    content: String,
    for_votes: u64,
    against_votes: u64,
    passed: bool,
}

impl Governance {
    pub fn new() -> Self {
        Self { proposals: HashMap::new(), next_id: 1 }
    }

    pub fn create_proposal(&mut self, content: String) -> u64 {
        let id = self.next_id;
        self.proposals.insert(id, Proposal {
            id, content, for_votes: 0, against_votes: 0, passed: false
        });
        self.next_id += 1;
        id
    }

    pub fn vote(&mut self, id: u64, approve: bool) {
        let prop = self.proposals.get_mut(&id).unwrap();
        if approve { prop.for_votes += 1 } else { prop.against_votes += 1 }
        prop.passed = prop.for_votes > prop.against_votes;
    }
}

fn main() {
    let mut gov = Governance::new();
    let id = gov.create_proposal("Increase Block Size".into());
    gov.vote(id, true);
    println!("Proposal: {:?}", gov.proposals.get(&id));
}
