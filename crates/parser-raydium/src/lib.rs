use bs58;
use instruction_parser::InstructionParser;
use types::{
    DecodedEvent, DecodedPumpFunCreatePoolEvent, DecodedPumpFunEvent, DecodedPumpFunSwapEvent,
    DecodedPumpFunSwapLog, StructuredInstruction, SwapEventAccounts, TransactionType, DecodedRaydiumSwapEvent
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
                |instruction| match self.decode_instruction(instruction, &account_keys) {
                    Some(decoded_instruction) => Some(DecodedEvent::PumpFun(decoded_instruction)),
                    None => None,
                },
            )
            .collect()
    }
}

impl RaydiumInstructionParser {
    const PROGRAM_ID: &'static str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
    const WSOL_ADDRESS: &'static str = "So11111111111111111111111111111111111111112";
    const POOL_CREATION_DISCRIMINATOR: [u8; 1] = [1];
    const SWAP_BASE_IN_DISCRIMINATOR: [u8; 1] = [9];

    pub fn decode_instruction(
        &self,
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> Option<DecodedPumpFunEvent> {
        let discriminator = &instruction.data[0..1];
        if discriminator == Self::SWAP_BASE_IN_DISCRIMINATOR {
            return Some(DecodedPumpFunEvent::Swap(Self::decode_buy_event(
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
        if data.len() < 8 {
            println!("instruction: {:?}", instruction)
        }
        let mut offset = 8;
        let name_length = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        if name_length > 100 {
            println!("{:?}", data);
        }
        offset += 4;
        let name = String::from_utf8(data[offset..offset + name_length as usize].to_vec()).unwrap();
        offset += name_length as usize;
        let symbol_length = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let symbol = String::from_utf8(data[offset..offset + symbol_length as usize].to_vec()).unwrap();
        offset += symbol_length as usize;
        let uri_length = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let uri = String::from_utf8(data[offset..offset + uri_length as usize].to_vec()).unwrap();
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
