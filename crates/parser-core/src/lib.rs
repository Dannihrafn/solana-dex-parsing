use instruction_parser::InstructionParser;
use parser_pump_amm::PumpAmmInstructionParser;
use parser_pumpfun::PumpFunInstructionParser;
use std::collections::{HashMap, HashSet};
use types::{DecodedEvent, StructuredInstruction};
use utils::{
    filter_instructions, get_account_keys,
    structure_all_instructions,
};
use yellowstone_grpc_proto::prelude::SubscribeUpdateTransaction;

#[derive(Clone, Debug)]
pub enum ParserEnum {
    PumpAmm(PumpAmmInstructionParser),
    PumpFun(PumpFunInstructionParser),
}

impl InstructionParser for ParserEnum {
    fn new() -> Self {
        ParserEnum::PumpAmm(PumpAmmInstructionParser::new())
    }

    fn get_program_id(&self) -> &str {
        match self {
            ParserEnum::PumpAmm(p) => p.get_program_id(),
            ParserEnum::PumpFun(p) => p.get_program_id(),
        }
    }

    fn decode_instructions(
        &self,
        instructions: Vec<StructuredInstruction>,
        account_keys: &Vec<String>,
        transaction: &SubscribeUpdateTransaction,
    ) -> Vec<DecodedEvent> {
        match self {
            ParserEnum::PumpAmm(p) => p.decode_instructions(instructions, account_keys, transaction),
            ParserEnum::PumpFun(p) => p.decode_instructions(instructions, account_keys, transaction),
        }
    }
}
#[derive(Clone, Debug)]
pub struct TransactionParser {
    parsers: HashMap<String, ParserEnum>,
    program_ids: HashSet<String>,
}

impl TransactionParser {
    pub fn new() -> Self {
        let mut parsers: HashMap<String, ParserEnum> = HashMap::new();
        let mut program_ids: HashSet<String> = HashSet::new();

        let pump_amm_parser = ParserEnum::PumpAmm(PumpAmmInstructionParser::new());
        let pump_fun_parser = ParserEnum::PumpFun(PumpFunInstructionParser::new());

        parsers.insert(
            pump_amm_parser.get_program_id().to_string(),
            pump_amm_parser.clone(),
        );
        parsers.insert(
            pump_fun_parser.get_program_id().to_string(),
            pump_fun_parser.clone(),
        );

        program_ids.insert(pump_amm_parser.get_program_id().to_string());
        program_ids.insert(pump_fun_parser.get_program_id().to_string());

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
        filter_instructions(
            &structured_instructions,
            &account_keys,
            &self.program_ids.clone(),
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
                    .decode_instructions(ids_and_ixs.get(key).unwrap().clone(), &account_keys, tx);
                ret.append(&mut results);
            }
        });
        ret
    }
}
