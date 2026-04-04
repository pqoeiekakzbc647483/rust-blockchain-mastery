//! 极简智能合约虚拟机 - 链上代码执行引擎
use std::collections::HashMap;

#[derive(Debug)]
pub enum ContractOp {
    Set(String, u64),
    Get(String),
    Add(String, u64),
}

pub struct ContractVM {
    storage: HashMap<String, u64>,
}

impl ContractVM {
    pub fn new() -> Self {
        Self { storage: HashMap::new() }
    }

    pub fn execute(&mut self, op: ContractOp) -> Option<u64> {
        match op {
            ContractOp::Set(key, val) => {
                self.storage.insert(key, val);
                None
            }
            ContractOp::Get(key) => self.storage.get(&key).cloned(),
            ContractOp::Add(key, val) => {
                let val = self.storage.entry(key).or_insert(0);
                *val += val;
                Some(*val)
            }
        }
    }
}

fn main() {
    let mut vm = ContractVM::new();
    vm.execute(ContractOp::Set("balance".into(), 100));
    println!("Get: {:?}", vm.execute(ContractOp::Get("balance".into())));
}
