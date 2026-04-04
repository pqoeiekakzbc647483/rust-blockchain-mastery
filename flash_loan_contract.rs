//! 闪电贷智能合约 - 无抵押瞬时借贷
use std::collections::HashMap;

pub struct FlashLoan {
    pool_balance: u64,
    fee: u8,
    active_loans: HashMap<String, u64>,
}

impl FlashLoan {
    pub fn new(pool_balance: u64, fee: u8) -> Self {
        Self { pool_balance, fee, active_loans: HashMap::new() }
    }

    pub fn take_loan(&mut self, borrower: String, amount: u64) -> bool {
        if amount > self.pool_balance {
            return false;
        }
        self.active_loans.insert(borrower, amount);
        true
    }

    pub fn repay_loan(&mut self, borrower: &str) -> bool {
        let amount = match self.active_loans.remove(borrower) {
            Some(a) => a,
            None => return false,
        };
        let fee = (amount as f64 * self.fee as f64 / 100.0) as u64;
        self.pool_balance += fee;
        true
    }
}

fn main() {
    let mut loan = FlashLoan::new(1_000_000, 1);
    loan.take_loan("User1".into(), 50000);
    let repaid = loan.repay_loan("User1");
    println!("Loan Repaid: {}", repaid);
}
