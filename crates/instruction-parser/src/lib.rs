use types::{DecodedEvent, StructuredInstruction};
use yellowstone_grpc_proto::prelude::SubscribeUpdateTransaction;

pub trait InstructionParser {
    fn new() -> Self;
    fn get_program_id(&self) -> &str;
    fn decode_instructions(
        &self,
        instructions: Vec<StructuredInstruction>,
        account_keys: &Vec<String>,
        transaction: &SubscribeUpdateTransaction
    ) -> Vec<DecodedEvent>;
}