use types::{
    DecodedPumpAmmBuyLog, MyTransaction, PumpAmmTransaction, StructuredInstruction, TransactionType,
};
use utils::{get_account_keys, get_filtered_instructions};

struct PumpAmmInstructionParser {}

impl PumpAmmInstructionParser {
    pub const PROGRAM_ID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
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

    pub fn decode_transaction(&self, transaction: &MyTransaction) -> Vec<PumpAmmTransaction> {
        let account_keys: Vec<String> = get_account_keys(transaction);
        let ixs: Vec<StructuredInstruction> =
            get_filtered_instructions(&account_keys, transaction, self.get_program_id());
        if ixs.is_empty() {
            return Vec::new();
        }
        let decoded_instructions: Vec<PumpAmmTransaction> = ixs
            .iter()
            .filter_map(|instruction| self.decode_instruction(instruction, &account_keys))
            .collect();
        decoded_instructions
    }

    pub fn decode_instruction(
        &self,
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> Option<PumpAmmTransaction> {
        let discriminator = &instruction.data[..8];
        if discriminator == Self::BUY_DISCRIMINATOR {
            return Self::decode_buy_event(instruction, account_keys);
        } else if discriminator == Self::SELL_DISCRIMINATOR {
            return Self::decode_sell_event(instruction, account_keys);
        } else if discriminator == Self::POOL_CREATION_DISCRIMINATOR {
            return Self::decode_pool_creation_event(instruction, account_keys);
        } else if discriminator == Self::WITHDRAW_DISCRIMINATOR {
            return Self::decode_withdraw_event(instruction, account_keys);
        } else if discriminator == Self::DEPOSIT_DISCRIMINATOR {
            return Self::decode_deposit_event(instruction, account_keys);
        }
        None
    }

    pub fn decode_buy_event(
        instruction: &StructuredInstruction,
        account_keys: &[String],
    ) -> Option<PumpAmmTransaction> {
        let account_key_indexes = &instruction.account_key_indexes;
        if account_key_indexes.is_empty() {
            return None;
        }
        let pool = account_keys[account_key_indexes[0] as usize].clone();
        let user = account_keys[account_key_indexes[1] as usize].clone();
        let base_mint = account_keys[account_key_indexes[3] as usize].clone();
        let quote_mint = account_keys[account_key_indexes[4] as usize].clone();

        let buy_log = instruction.inner_instructions.last();

        todo!();
    }

    pub fn decode_buy_log(data: &[u8]) -> Option<DecodedPumpAmmBuyLog> {
        if data.len() < 280 {
            return None;
        }
        let mut offset: usize = 24;
        let base_amount_out: u64 = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let pool_base_token_reserves: u64 =
            u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let pool_quote_token_reserves: u64 =
            u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;
        let quote_amount_in: u64 = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 248;
        let coin_creator: String = String::from_utf8(data[offset..offset + 32].to_vec()).unwrap();
        return Some(DecodedPumpAmmBuyLog {
            quote_amount_in,
            base_amount_out,
            pool_base_token_reserves,
            pool_quote_token_reserves,
            coin_creator,
            transaction_type: TransactionType::Buy,
        });
    }

    pub fn decode_sell_event(
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> Option<PumpAmmTransaction> {
        todo!();
    }

    pub fn decode_pool_creation_event(
        instruction: &StructuredInstruction,
        account_keys: &Vec<String>,
    ) -> Option<PumpAmmTransaction> {
        todo!();
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
