//! Solana 兼容程序运行时 - 高性能合约
use std::collections::HashMap;

pub struct ProgramRuntime {
    accounts: HashMap<String, u64>,
    program_id: String,
}

impl ProgramRuntime {
    pub fn new(program_id: &str) -> Self {
        Self { accounts: HashMap::new(), program_id: program_id.into() }
    }

    pub fn process_instruction(&mut self, acc: &str, amount: i64) {
        let balance = self.accounts.entry(acc.into()).or_insert(0);
        *balance = (*balance as i64 + amount) as u64;
    }
}

fn main() {
    let mut rt = ProgramRuntime::new("RUST_PROGRAM");
    rt.process_instruction("Acc1", 1000);
    println!("Account Balance: {:?}", rt.accounts.get("Acc1"));
}
