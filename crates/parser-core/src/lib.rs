use types::{MyTransaction, TransactionEvent};
use yellowstone_grpc_proto::prelude::SubscribeUpdateTransaction;

pub enum ParsedTransaction {
    PumpAmm(ParsedPumpAmmTransaction),
    PumpFun(ParsedPumpFunTransaction)
}

pub trait TransactionParserNew {
    fn parse_transaction(&self, transaction: SubscribeUpdateTransaction) -> Vec<ParsedTransaction>;
}