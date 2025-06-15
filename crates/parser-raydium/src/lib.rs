use bs58;
use instruction_parser::InstructionParser;
use types::{
    DecodedEvent, StructuredInstruction, DecodedRaydiumSwapEvent, DecodedRaydiumCreatePoolEvent, DecodedRaydiumEvent
};
use yellowstone_grpc_proto::prelude::SubscribeUpdateTransaction;
use utils::parse_token_program_transfer;

#[derive(Clone, Debug)]
pub struct RaydiumInstructionParser {}

impl InstructionParser for RaydiumInstructionParser {
    fn new() -> Self {
        Self {}
    }

    fn get_program_id(&self) -> &str {
        Self::PROGRAM_ID
    }

    fn decode_instructions(
        &self,
        instructions: Vec<StructuredInstruction>,
        account_keys: &Vec<String>,
        transaction: &SubscribeUpdateTransaction,
    ) -> Vec<DecodedEvent> {
        instructions
            .iter()
            .filter_map(
                |instruction| match self.decode_instruction(instruction, &account_keys, transaction) {
                    Some(decoded_instruction) => Some(DecodedEvent::Raydium(decoded_instruction)),
                    None => None,
                },
            )
            .collect()
    }
}

impl RaydiumInstructionParser {
    const PROGRAM_ID: &'static str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
    const TOKEN_PROGRAM_ID: &'static str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    const WSOL_ADDRESS: &'static str = "So11111111111111111111111111111111111111112";
    const POOL_CREATION_DISCRIMINATOR: [u8; 1] = [1];
    const SWAP_BASE_IN_DISCRIMINATOR: [u8; 1] = [9];

    pub fn decode_instruction(
        &self,
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
        transaction: &SubscribeUpdateTransaction,
    ) -> Option<DecodedRaydiumEvent> {
        let discriminator = &instruction.data[0..1];
        if discriminator == Self::SWAP_BASE_IN_DISCRIMINATOR {
            return Some(DecodedRaydiumEvent::Swap(Self::decode_swap_base_in(
                instruction, account_keys, transaction
            )));
        } else if discriminator == Self::POOL_CREATION_DISCRIMINATOR {
            return Some(DecodedRaydiumEvent::CreatePool(
                Self::decode_pool_creation_event(instruction, account_keys),
            ));
        }
        None
    }

    pub fn decode_pool_creation_event(
        instruction: &StructuredInstruction,
        account_keys: &[String],
    ) -> DecodedRaydiumCreatePoolEvent {
        let account_key_indexes = &instruction.account_key_indexes;
        let inner_instructions = &instruction.inner_instructions;
        let token_program_index = account_keys.iter().position(|key| key == Self::TOKEN_PROGRAM_ID).unwrap();
        let token_program_transactions: Vec<&StructuredInstruction> = inner_instructions.iter().filter(|instruction| instruction.program_id_index == token_program_index as u8).collect();
        let base_mint_transfer = parse_token_program_transfer(token_program_transactions[0], &account_keys.to_vec());
        let quote_mint_transfer =  parse_token_program_transfer(token_program_transactions[1], &account_keys.to_vec());

        let user = account_keys[account_key_indexes[0] as usize].clone();
        let pool = account_keys[account_key_indexes[4] as usize].clone();
        let base_mint = account_keys[account_key_indexes[8] as usize].clone();
        let quote_mint = account_keys[account_key_indexes[9] as usize].clone();

        DecodedRaydiumCreatePoolEvent {
            pool,
            user,
            base_mint,
            quote_mint,
            base_amount: base_mint_transfer.amount,
            quote_amount: quote_mint_transfer.amount,
        }

    }

    pub fn decode_swap_base_in(instruction: &StructuredInstruction, account_keys: &Vec<String>, transaction: &SubscribeUpdateTransaction) -> DecodedRaydiumSwapEvent {
        let account_key_indexes = &instruction.account_key_indexes;
        let inner_instructions = &instruction.inner_instructions;
        let in_transfer = parse_token_program_transfer(&inner_instructions[0], account_keys);
        let out_transfer = parse_token_program_transfer(&inner_instructions[1], account_keys);

        let pre_token_balances = transaction.clone().transaction.unwrap().meta.unwrap().pre_token_balances;
        let post_token_balances = transaction.clone().transaction.unwrap().meta.unwrap().post_token_balances;

        let mut out_token_balance;
        let mut in_token_balance;

        match post_token_balances.iter().find(|balance| balance.account_index == inner_instructions[0].account_key_indexes[1] as u32) {
            Some(balance) =>
                in_token_balance = balance.clone(),
            None => {
                match pre_token_balances.iter().find(|balance| balance.account_index == inner_instructions[0].account_key_indexes[1] as u32) {
                    Some(balance) => in_token_balance = balance.clone(),
                    None => panic!("No in_token_balance could be found")
                }
            }
        }
        match post_token_balances.iter().find(|balance| balance.account_index == inner_instructions[1].account_key_indexes[0] as u32) {
            Some(balance) => out_token_balance = balance.clone(),
            None => {
                match pre_token_balances.iter().find(|balance| balance.account_index == inner_instructions[1].account_key_indexes[0] as u32) {
                    Some(balance) => out_token_balance = balance.clone(),
                    None => panic!("No out_token_balance could be found")
                }
            }
        }

        let pool = account_keys[account_key_indexes[1] as usize].clone();
        let user = in_transfer.authority;
        let mint_in = in_token_balance.clone().mint;
        let mint_out = out_token_balance.clone().mint;
        let amount_in = in_transfer.amount;
        let amount_out = out_transfer.amount;
        let mint_in_reserve = in_token_balance.clone().ui_token_amount.unwrap().amount.parse::<u64>().unwrap();
        let mint_out_reserve = out_token_balance.clone().ui_token_amount.unwrap().amount.parse::<u64>().unwrap();
        let in_decimals = in_token_balance.ui_token_amount.unwrap().decimals as u8;
        let out_decimals = out_token_balance.ui_token_amount.unwrap().decimals as u8;

        DecodedRaydiumSwapEvent {
            pool,
            user,
            mint_in,
            mint_out,
            amount_in,
            amount_out,
            mint_in_reserve,
            mint_out_reserve,
            in_decimals,
            out_decimals
        }
    }
}
