use bs58;
use types::{
    DecodedEvent, DecodedPumpFunCreatePoolEvent, DecodedPumpFunEvent, DecodedPumpFunSwapEvent,
    DecodedPumpFunSwapLog, StructuredInstruction, SwapEventAccounts, TransactionType,
};
use utils::{get_account_keys, get_filtered_instructions};
use yellowstone_grpc_proto::prelude::SubscribeUpdateTransaction;
use instruction_parser::InstructionParser;
pub struct PumpFunInstructionParser {}

impl InstructionParser for PumpFunInstructionParser {
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
    ) -> Vec<DecodedEvent> {
        instructions
            .iter()
            .filter_map(
                |instruction| match self.decode_instruction(instruction, &account_keys) {
                    Some(decoded_instruction) => Some(DecodedEvent::PumpFun(decoded_instruction)),
                    None => None,
                },
            )
            .collect()
    }
}

impl PumpFunInstructionParser {
    const PROGRAM_ID: &'static str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
    const WSOL_ADDRESS: &'static str = "So11111111111111111111111111111111111111112";
    const POOL_CREATION_DISCRIMINATOR: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
    const BUY_DISCRIMINATOR: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
    const SELL_DISCRIMINATOR: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

    pub fn decode_transaction(
        &self,
        transaction: &SubscribeUpdateTransaction,
    ) -> Vec<DecodedPumpFunEvent> {
        let account_keys: Vec<String> = get_account_keys(transaction);
        let ixs: Vec<StructuredInstruction> =
            get_filtered_instructions(transaction, &account_keys, self.get_program_id());
        if ixs.is_empty() {
            return Vec::new();
        }
        let decoded_instructions: Vec<DecodedPumpFunEvent> = ixs
            .iter()
            .filter_map(|instruction| self.decode_instruction(instruction, &account_keys))
            .collect();
        decoded_instructions
    }

    pub fn decode_instruction(
        &self,
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> Option<DecodedPumpFunEvent> {
        let discriminator = &instruction.data[..8];
        if discriminator == Self::BUY_DISCRIMINATOR {
            return Some(DecodedPumpFunEvent::Swap(Self::decode_buy_event(
                instruction,
            )));
        } else if discriminator == Self::SELL_DISCRIMINATOR {
            return Some(DecodedPumpFunEvent::Swap(Self::decode_sell_event(
                instruction,
            )));
        } else if discriminator == Self::POOL_CREATION_DISCRIMINATOR {
            return Some(DecodedPumpFunEvent::CreatePool(
                Self::decode_pool_creation_event(instruction, account_keys),
            ));
        }
        None
    }

    pub fn decode_buy_event(instruction: &StructuredInstruction) -> DecodedPumpFunSwapEvent {
        let last_ix = instruction.inner_instructions.last().unwrap();
        let buy_log = if last_ix.data.len() < 233 {
            instruction
                .inner_instructions
                .get(instruction.inner_instructions.len().wrapping_sub(2))
                .unwrap()
        } else {
            last_ix
        };
        let decoded_buy_log = Self::decode_buy_log(&buy_log.data);

        DecodedPumpFunSwapEvent {
            accounts: SwapEventAccounts {
                pool: decoded_buy_log.mint.clone(),
                user: decoded_buy_log.user.clone(),
                base_mint: decoded_buy_log.mint.clone(),
                quote_mint: Self::WSOL_ADDRESS.to_string(),
            },
            mint_in: Self::WSOL_ADDRESS.to_string(),
            mint_out: decoded_buy_log.mint.clone(),
            amount_in: decoded_buy_log.sol_amount,
            amount_out: decoded_buy_log.token_amount,
            mint_in_reserve: decoded_buy_log.virtual_sol_reserves,
            mint_out_reserve: decoded_buy_log.virtual_token_reserves,
            event_type: TransactionType::Buy,
        }
    }

    pub fn decode_buy_log(data: &[u8]) -> DecodedPumpFunSwapLog {
        let mut offset: usize = 16;
        let mint: String = bs58::encode(data[offset..offset + 32].to_vec()).into_string();
        offset += 32;
        let sol_amount = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let token_amount = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 9;
        let user = bs58::encode(data[offset..offset + 32].to_vec()).into_string();
        offset += 40;
        let virtual_sol_reserves = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let virtual_token_reserves =
            u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());

        DecodedPumpFunSwapLog {
            mint,
            sol_amount,
            token_amount,
            user,
            virtual_sol_reserves,
            virtual_token_reserves,
        }
    }

    pub fn decode_sell_event(instruction: &StructuredInstruction) -> DecodedPumpFunSwapEvent {
        let inner_instructions = &instruction.inner_instructions;
        let last_ix = &inner_instructions.last().unwrap();
        let sell_log = if last_ix.data.len() < 233 {
            inner_instructions
                .get(inner_instructions.len().wrapping_sub(2))
                .unwrap()
        } else {
            last_ix
        };
        let decoded_sell_log = Self::decode_sell_log(&sell_log.data);

        DecodedPumpFunSwapEvent {
            accounts: SwapEventAccounts {
                pool: decoded_sell_log.mint.clone(),
                user: decoded_sell_log.user.clone(),
                base_mint: decoded_sell_log.mint.clone(),
                quote_mint: Self::WSOL_ADDRESS.to_string(),
            },
            mint_in: decoded_sell_log.mint.clone(),
            mint_out: Self::WSOL_ADDRESS.to_string(),
            amount_in: decoded_sell_log.token_amount,
            amount_out: decoded_sell_log.sol_amount,
            mint_in_reserve: decoded_sell_log.virtual_token_reserves,
            mint_out_reserve: decoded_sell_log.virtual_sol_reserves,
            event_type: TransactionType::Sell,
        }
    }

    fn decode_sell_log(data: &[u8]) -> DecodedPumpFunSwapLog {
        let mut offset: usize = 16;
        let mint: String = bs58::encode(data[offset..offset + 32].to_vec()).into_string();
        offset += 32;
        let sol_amount = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let token_amount = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 9;
        let user = bs58::encode(data[offset..offset + 32].to_vec()).into_string();
        offset += 40;
        let virtual_sol_reserves = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let virtual_token_reserves =
            u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());

        DecodedPumpFunSwapLog {
            mint,
            sol_amount,
            token_amount,
            user,
            virtual_sol_reserves,
            virtual_token_reserves,
        }
    }

    pub fn decode_pool_creation_event(
        instruction: &StructuredInstruction,
        account_keys: &[String],
    ) -> DecodedPumpFunCreatePoolEvent {
        let account_key_indexes: &Vec<u8> = &instruction.account_key_indexes;
        let data: &Vec<u8> = &instruction.data;
        let mut offset = 8;
        let name_length = u32::from_le_bytes(data[0..4].try_into().unwrap());
        offset += 4;
        let name = String::from_utf8(data[offset..4 + name_length as usize].to_vec()).unwrap();
        offset += name_length as usize;
        let symbol_length = u32::from_le_bytes(data[offset..4].try_into().unwrap());
        offset += 4;
        let symbol = String::from_utf8(data[offset..4 + symbol_length as usize].to_vec()).unwrap();
        offset += symbol_length as usize;
        let uri_length = u32::from_le_bytes(data[offset..4].try_into().unwrap());
        offset += 4;
        let uri = String::from_utf8(data[offset..4 + uri_length as usize].to_vec()).unwrap();
        offset += uri_length as usize;
        let coin_creator = bs58::encode(data[offset..offset + 32].to_vec()).into_string();

        let mint =
            bs58::encode(account_keys[account_key_indexes[0] as usize].clone()).into_string();
        let bonding_curve =
            bs58::encode(account_keys[account_key_indexes[2] as usize].clone()).into_string();
        let associated_bonding_curve =
            bs58::encode(account_keys[account_key_indexes[3] as usize].clone()).into_string();

        DecodedPumpFunCreatePoolEvent {
            name,
            symbol,
            uri,
            creator: coin_creator,
            base_mint: mint,
            quote_mint: Self::WSOL_ADDRESS.to_string(),
            bonding_curve,
            associated_bonding_curve,
            event_type: TransactionType::CreatePool,
        }
    }
}
