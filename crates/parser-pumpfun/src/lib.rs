use bs58;
use types::{MyTransaction, StructuredInstruction, TransactionType};
use utils::{get_account_keys, get_filtered_instructions};

#[derive(Debug)]
pub struct DecodedPumpFunSwapLog {
    mint: String,
    sol_amount: u64,
    token_amount: u64,
    user: String,
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
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
pub struct DecodedPumpFunCreatePoolEvent {
    name: String,
    symbol: String,
    uri: String,
    creator: String,
    base_mint: String,
    quote_mint: String,
    bonding_curve: String,
    associated_bonding_curve: String,
    event_type: TransactionType
}

#[derive(Debug)]
pub enum DecodedEvent {
    Swap(DecodedSwapEvent),
    CreatePool(DecodedPumpFunCreatePoolEvent),
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
pub struct DecodedSwapEvent {
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
    pub const PROGRAM_ID: &'static str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
    pub const WSOL_ADDRESS: &'static str = "So11111111111111111111111111111111111111112";
    pub const POOL_CREATION_DISCRIMINATOR: [u8; 8] = [
        24, 30, 200, 40, 5, 28, 7, 119,
    ];
    pub const BUY_DISCRIMINATOR: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
    pub const SELL_DISCRIMINATOR: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_program_id(&self) -> &str {
        Self::PROGRAM_ID
    }

    pub fn decode_transaction(&self, transaction: &MyTransaction) -> Vec<DecodedEvent> {
        let account_keys: Vec<String> = get_account_keys(transaction);
        let ixs: Vec<StructuredInstruction> =
            get_filtered_instructions(&account_keys, transaction, self.get_program_id());
        if ixs.is_empty() {
            return Vec::new();
        }
        let decoded_instructions: Vec<DecodedEvent> = ixs
            .iter()
            .filter_map(|instruction| self.decode_instruction(instruction, &account_keys))
            .collect();
        decoded_instructions
    }

    pub fn decode_instruction(
        &self,
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> Option<DecodedEvent> {
        let discriminator = &instruction.data[..8];
        if discriminator == Self::BUY_DISCRIMINATOR {
            return Some(DecodedEvent::Swap(Self::decode_buy_event(
                instruction,
            )));
        } else if discriminator == Self::SELL_DISCRIMINATOR {
            return Some(DecodedEvent::Swap(Self::decode_sell_event(
                instruction,
            )));
        } else if discriminator == Self::POOL_CREATION_DISCRIMINATOR {
            return Some(DecodedEvent::CreatePool(Self::decode_pool_creation_event(
                instruction,
                account_keys,
            )));
        }
        None
    }

    pub fn decode_buy_event(
        instruction: &StructuredInstruction,
    ) -> DecodedSwapEvent {
        let last_ix = instruction.inner_instructions.last().unwrap();
        let buy_log = if last_ix.data.len() < 233 {instruction.inner_instructions.get(instruction.inner_instructions.len().wrapping_sub(2)).unwrap()} else {last_ix};
        let decoded_buy_log = Self::decode_buy_log(&buy_log.data);

        DecodedSwapEvent {
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
        let virtual_token_reserves = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());

        DecodedPumpFunSwapLog {
            mint,
            sol_amount,
            token_amount,
            user,
            virtual_sol_reserves,
            virtual_token_reserves,
        }
    }

    pub fn decode_sell_event(
        instruction: &StructuredInstruction,
    ) -> DecodedSwapEvent {
        let inner_instructions = &instruction.inner_instructions;
        let last_ix = &inner_instructions.last().unwrap();
        let sell_log = if last_ix.data.len() < 233 {inner_instructions.get(inner_instructions.len().wrapping_sub(2)).unwrap()} else {last_ix};
        let decoded_sell_log = Self::decode_sell_log(&sell_log.data);

        DecodedSwapEvent {
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
        let virtual_token_reserves = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());

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
        let name = String::from_utf8(data[offset.. 4 + name_length as usize].to_vec()).unwrap();
        offset += name_length as usize;
        let symbol_length = u32::from_le_bytes(data[offset..4].try_into().unwrap());
        offset += 4;
        let symbol = String::from_utf8(data[offset.. 4 + symbol_length as usize].to_vec()).unwrap();
        offset += symbol_length as usize;
        let uri_length = u32::from_le_bytes(data[offset..4].try_into().unwrap());
        offset += 4;
        let uri = String::from_utf8(data[offset.. 4 + uri_length as usize].to_vec()).unwrap();
        offset += uri_length as usize;
        let coin_creator = bs58::encode(data[offset..offset + 32].to_vec()).into_string();

        let mint = bs58::encode(account_keys[account_key_indexes[0] as usize].clone()).into_string();
        let bonding_curve = bs58::encode(account_keys[account_key_indexes[2] as usize].clone()).into_string();
        let associated_bonding_curve = bs58::encode(account_keys[account_key_indexes[3] as usize].clone()).into_string();
        let user = bs58::encode(account_keys[account_key_indexes[7] as usize].clone()).into_string();

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
