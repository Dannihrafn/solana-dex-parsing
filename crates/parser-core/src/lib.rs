use types::{MyTransaction, TransactionEvent};

pub trait TransactionParser {
    fn parse_transaction(transaction: MyTransaction) -> Vec<TransactionEvent>;
}
