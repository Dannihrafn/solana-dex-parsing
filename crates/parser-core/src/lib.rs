use types::{MyTransaction, TransactionEvent};
use yellowstone_grpc_proto::prelude::SubscribeUpdateTransaction;
use types::{DecodedPumpAmmEvent, DecodedPumpFunEvent};
use std::collections::{HashSet, HashMap};
use utils::get_account_keys;

pub enum DecodedEvent {
    PumpAmm(DecodedPumpAmmEvent),
    PumpFun(DecodedPumpFunEvent)
}
pub trait InstructionParser {}



pub struct TransactionParser {
    parsers: HashMap<String, Box<dyn InstructionParser>>,
}

impl TransactionParser {
    pub fn new() -> Self {
        let mut parsers: HashMap<String, Box<dyn InstructionParser>> = HashMap::new();

        let pump_amm_parser = Box::new(PumpAmmInstructionParser);
        let pump_fun_parser = Box::new(PumpFunInstructionParser);

        parsers.insert(pump_amm_parser.program_id().to_string(), pump_amm_parser);
        parsers.insert(pump_fun_parser.program_id().to_string(), pump_fun_parser);

        Self { parsers }
    }

    fn get_parsers(&self, tx: &SubscribeUpdateTransaction) -> Vec<&dyn InstructionParser> {
        let account_keys = get_account_keys(tx);
        let mut unique_parsers: HashSet<*const dyn InstructionParser> = HashSet::new();
        let mut parsers: Vec<&dyn InstructionParser> = Vec::new();

        for key in account_keys {
            if let Some(parser) = self.parsers.get(&key) {
                let parser_ptr = parser.as_ref() as *const dyn InstructionParser;
                if unique_parsers.insert(parser_ptr) {
                    parsers.push(parser.as_ref());
                }
            }
        }

        parsers
    }

    pub fn decode_transaction(&self, tx: &SubscribeUpdateTransaction) -> Vec<DecodedEvent> {
        let parsers = self.get_parsers(tx);
        let mut decoded_instructions = Vec::new();

        for parser in parsers {
            let mut results = parser.decode_transaction(tx);
            decoded_instructions.append(&mut results);
        }

        decoded_instructions
    }
}
pub trait TransactionParser_ {
    fn parse_transaction(&self, transaction: SubscribeUpdateTransaction) -> Vec<DecodedEvent>;
    fn get_program_id(&self) -> String;
}