use types::{DecodedEvent, StructuredInstruction};

pub trait InstructionParser {
    fn new() -> Self;
    fn get_program_id(&self) -> &str;
    fn decode_instructions(
        &self,
        instructions: Vec<StructuredInstruction>,
        account_keys: &Vec<String>,
    ) -> Vec<DecodedEvent>;
}