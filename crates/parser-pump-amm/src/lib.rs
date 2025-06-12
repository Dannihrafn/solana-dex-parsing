use bs58;
use types::{PumpAmmTransaction, StructuredInstruction, TransactionType};
use utils::{get_account_keys, get_filtered_instructions};
use yellowstone_grpc_proto::prelude::SubscribeUpdateTransaction;

#[derive(Debug)]
pub struct DecodedPumpAmmBuyLog {
    quote_amount_in: u64,
    base_amount_out: u64,
    pool_base_token_reserves: u64,
    pool_quote_token_reserves: u64,
    coin_creator: String,
}

#[derive(Debug)]
pub struct DecodedPumpAmmSellLog {
    base_amount_in: u64,
    pool_base_token_reserves: u64,
    pool_quote_token_reserves: u64,
    quote_amount_out: u64,
    coin_creator: String,
}
#[derive(Debug)]
pub struct DecodedPumpAmmCreatePoolEvent {
    pool: String,
    creator: String,
    base_mint: String,
    quote_mint: String,
    pool_base_token_reserve: u64,
    pool_quote_token_reserve: u64,
    pool_base_token_account: String,
    pool_quote_token_account: String,
    index: u16,
    event_type: TransactionType,
}

#[derive(Debug)]
pub enum DecodedPumpAmmEvent {
    Swap(DecodedPumpAmmSwapEvent),
    CreatePool(DecodedPumpAmmCreatePoolEvent),
    //Withdraw(DecodedWithdrawEvent),
    //Deposit(DecodedDepositEvent),
}

#[derive(Debug)]
struct SwapEventAccounts {
    pool: String,
    user: String,
    base_mint: String,
    quote_mint: String,
}

#[derive(Debug)]
pub struct DecodedPumpAmmSwapEvent {
    accounts: SwapEventAccounts,
    mint_in: String,
    mint_out: String,
    amount_in: u64,
    amount_out: u64,
    mint_in_reserve: u64,
    mint_out_reserve: u64,
    event_type: TransactionType,
}

pub struct PumpAmmInstructionParser {}

impl PumpAmmInstructionParser {
    pub const PROGRAM_ID: &'static str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
    pub const POOL_CREATION_DISCRIMINATOR: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
    pub const BUY_DISCRIMINATOR: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
    pub const SELL_DISCRIMINATOR: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
    pub const DEPOSIT_DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
    pub const WITHDRAW_DISCRIMINATOR: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_program_id(&self) -> &str {
        Self::PROGRAM_ID
    }

    pub fn decode_transaction(&self, transaction: &SubscribeUpdateTransaction) -> Vec<DecodedPumpAmmEvent> {
        let account_keys: Vec<String> = get_account_keys(transaction);
        let ixs: Vec<StructuredInstruction> =
            get_filtered_instructions(transaction, &account_keys, self.get_program_id());
        if ixs.is_empty() {
            return Vec::new();
        }
        let decoded_instructions: Vec<DecodedPumpAmmEvent> = ixs
            .iter()
            .filter_map(|instruction| self.decode_instruction(instruction, &account_keys))
            .collect();
        decoded_instructions
    }

    pub fn decode_instruction(
        &self,
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> Option<DecodedPumpAmmEvent> {
        let discriminator = &instruction.data[..8];
        if discriminator == Self::BUY_DISCRIMINATOR {
            return Some(DecodedPumpAmmEvent::Swap(Self::decode_buy_event(
                instruction,
                account_keys,
            )));
        } else if discriminator == Self::SELL_DISCRIMINATOR {
            return Some(DecodedPumpAmmEvent::Swap(Self::decode_sell_event(
                instruction,
                account_keys,
            )));
        } else if discriminator == Self::POOL_CREATION_DISCRIMINATOR {
            return Some(DecodedPumpAmmEvent::CreatePool(Self::decode_pool_creation_event(
                instruction,
                account_keys,
            )));
        } /*else if discriminator == Self::WITHDRAW_DISCRIMINATOR {
        return Self::decode_withdraw_event(instruction, account_keys);
        } else if discriminator == Self::DEPOSIT_DISCRIMINATOR {
        return Self::decode_deposit_event(instruction, account_keys);
        }*/
        None
    }

    pub fn decode_buy_event(
        instruction: &StructuredInstruction,
        account_keys: &[String],
    ) -> DecodedPumpAmmSwapEvent {
        let account_key_indexes = &instruction.account_key_indexes;
        let pool = account_keys[account_key_indexes[0] as usize].clone();
        let user = account_keys[account_key_indexes[1] as usize].clone();
        let base_mint = account_keys[account_key_indexes[3] as usize].clone();
        let quote_mint = account_keys[account_key_indexes[4] as usize].clone();
        let buy_log = instruction.inner_instructions.last().unwrap();
        let decoded_buy_log = Self::decode_buy_log(&buy_log.data).unwrap();
        let mint_in_reserve = decoded_buy_log.pool_base_token_reserves.clone();
        let mint_out_reserve = decoded_buy_log.pool_quote_token_reserves.clone();

        DecodedPumpAmmSwapEvent {
            accounts: SwapEventAccounts {
                pool,
                user,
                base_mint: base_mint.clone(),
                quote_mint: quote_mint.clone(),
            },
            mint_in: quote_mint,
            mint_out: base_mint,
            amount_in: decoded_buy_log.quote_amount_in,
            amount_out: decoded_buy_log.base_amount_out,
            mint_in_reserve,
            mint_out_reserve,
            event_type: TransactionType::Buy,
        }
    }

    pub fn decode_buy_log(data: &[u8]) -> Option<DecodedPumpAmmBuyLog> {
        if data.len() < 352 {
            return None;
        }
        let mut offset: usize = 24;
        let base_amount_out: u64 = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 32;
        let pool_base_token_reserves: u64 =
            u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let pool_quote_token_reserves: u64 =
            u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let quote_amount_in: u64 = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 248;
        let coin_creator: String = bs58::encode(data[offset..offset + 32].to_vec()).into_string();
        return Some(DecodedPumpAmmBuyLog {
            quote_amount_in,
            base_amount_out,
            pool_base_token_reserves,
            pool_quote_token_reserves,
            coin_creator,
        });
    }

    pub fn decode_sell_event(
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> DecodedPumpAmmSwapEvent {
        let account_key_indexes = &instruction.account_key_indexes;
        let pool = account_keys[account_key_indexes[0] as usize].clone();
        let user = account_keys[account_key_indexes[1] as usize].clone();
        let base_mint = account_keys[account_key_indexes[3] as usize].clone();
        let quote_mint = account_keys[account_key_indexes[4] as usize].clone();
        let sell_log = instruction.inner_instructions.last().unwrap();
        let decoded_sell_log = Self::decode_sell_log(&sell_log.data).unwrap();
        let mint_in_reserve = decoded_sell_log.pool_base_token_reserves.clone();
        let mint_out_reserve = decoded_sell_log.pool_quote_token_reserves.clone();

        DecodedPumpAmmSwapEvent {
            accounts: SwapEventAccounts {
                pool,
                user,
                base_mint: base_mint.clone(),
                quote_mint: quote_mint.clone(),
            },
            mint_in: base_mint,
            mint_out: quote_mint,
            amount_in: decoded_sell_log.base_amount_in,
            amount_out: decoded_sell_log.quote_amount_out,
            mint_in_reserve,
            mint_out_reserve,
            event_type: TransactionType::Sell,
        }
    }

    fn decode_sell_log(data: &[u8]) -> Option<DecodedPumpAmmSellLog> {
        let mut offset: usize = 24;
        let base_amount_in: u64 = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 32;
        let pool_base_token_reserves: u64 =
            u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let pool_quote_token_reserves: u64 =
            u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 56;
        let quote_amount_out: u64 =
            u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 200;
        let coin_creator: String = bs58::encode(data[offset..offset + 32].to_vec()).into_string();
        return Some(DecodedPumpAmmSellLog {
            base_amount_in,
            pool_base_token_reserves,
            pool_quote_token_reserves,
            quote_amount_out,
            coin_creator,
        });
    }

    pub fn decode_pool_creation_event(
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> DecodedPumpAmmCreatePoolEvent {
        let account_key_indexes: &Vec<u8> = &instruction.account_key_indexes;
        let data: &Vec<u8> = &instruction.data;
        let pool: String = account_keys[account_key_indexes[0] as usize].clone();
        let base_mint: String = account_keys[account_key_indexes[3] as usize].clone();
        let quote_mint: String = account_keys[account_key_indexes[4] as usize].clone();
        let pool_base_token_account: String = account_keys[account_key_indexes[9] as usize].clone();
        let pool_quote_token_account: String =
            account_keys[account_key_indexes[10] as usize].clone();

        let mut offset: usize = 8;
        let index: u16 = u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap());
        offset += 2;
        let base_amount_in: u64 = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let quote_amount_in: u64 = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let coin_creator: String = bs58::encode(data[offset..offset + 32].to_vec()).into_string();

        DecodedPumpAmmCreatePoolEvent {
            pool,
            creator: coin_creator,
            base_mint,
            quote_mint,
            pool_base_token_reserve: base_amount_in,
            pool_quote_token_reserve: quote_amount_in,
            pool_base_token_account,
            pool_quote_token_account,
            index,
            event_type: TransactionType::CreatePool,
        }
    }

    pub fn decode_withdraw_event(
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> Option<PumpAmmTransaction> {
        todo!();
    }

    pub fn decode_deposit_event(
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> Option<PumpAmmTransaction> {
        todo!();
    }
}
