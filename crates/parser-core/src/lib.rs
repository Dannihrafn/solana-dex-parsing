use parser_pump_amm::PumpAmmInstructionParser;
use parser_pumpfun::PumpFunInstructionParser;
use std::collections::{HashMap, HashSet};
use types::{DecodedEvent, StructuredInstruction};
use utils::{filter_instructions_new, get_account_keys, structure_all_instructions};
use yellowstone_grpc_proto::prelude::SubscribeUpdateTransaction;
use instruction_parser::InstructionParser;

pub struct TransactionParserNew {
    parsers: HashMap<String, Box<dyn InstructionParser>>,
    program_ids: HashSet<String>,
}

impl TransactionParserNew {
    pub fn new() -> Self {
        let mut parsers: HashMap<String, Box<dyn InstructionParser>> = HashMap::new();
        let mut program_ids: HashSet<String> = HashSet::new();

        let pump_amm_parser = Box::new(PumpAmmInstructionParser);
        let pump_fun_parser = Box::new(PumpFunInstructionParser);

        parsers.insert(pump_amm_parser.program_id().to_string(), pump_amm_parser);
        parsers.insert(pump_fun_parser.program_id().to_string(), pump_fun_parser);

        program_ids.insert(pump_amm_parser.program_id().to_string());
        program_ids.insert(pump_fun_parser.program_id().to_string());

        Self {
            parsers,
            program_ids,
        }
    }

    pub fn get_parsers_and_instructions(
        &self,
        tx: &SubscribeUpdateTransaction,
        account_keys: &Vec<String>,
    ) -> HashMap<String, Vec<StructuredInstruction>> {
        let structured_instructions = structure_all_instructions(tx);
        filter_instructions_new(
            &structured_instructions,
            &account_keys,
            self.program_ids.clone(),
        )
    }

    pub fn decode_transaction(&self, tx: &SubscribeUpdateTransaction) -> Vec<DecodedEvent> {
        let mut ret: Vec<DecodedEvent> = vec![];
        let account_keys = get_account_keys(tx);
        let ids_and_ixs = self.get_parsers_and_instructions(tx, &account_keys);
        let keys = ids_and_ixs.keys().cloned().collect::<Vec<String>>();
        keys.iter().for_each(|key| {
            if let Some(parser) = self.parsers.get(key) {
                let mut results = parser
                    .decode_instructions(ids_and_ixs.get(key).unwrap().clone(), &account_keys);
                ret.append(&mut results);
            }
        });
        ret
    }
}
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
