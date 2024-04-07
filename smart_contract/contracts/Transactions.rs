use serde::Serialize;
use std::collections::VecDeque;

#[derive(Serialize  Debug)]
struct Transfer {
    from: String,
    receiver: String,
    amount: u64,
    message: String,
    timestamp: u64,
    keyword: String,
}

#[derive(Default, Serialize, Debug)]
struct Transactions {
    transaction_count: u64,
    transactions: VecDeque<Transfer>,
}

impl Transactions {
    fn new() -> Self {
        Default::default()
    }

    fn add_to_blockchain(
        &mut self,
        receiver: String,
        amount: u64,
        message: String,
        keyword: String,
    ) -> Transfer {
        self.transaction_count += 1;
        let transfer = Transfer {
            from: "sender_address".to_string(),
            receiver,
            amount,
            message,
            timestamp: chrono::Utc::now().timestamp() as u64,
            keyword,
        };
        self.transactions.push_back(transfer.clone());
        transfer
    }

    fn get_all_transactions(&self) -> &VecDeque<Transfer> {
        &self.transactions
    }

    fn get_transaction_count(&self) -> u64 {
        self.transaction_count
    }
}