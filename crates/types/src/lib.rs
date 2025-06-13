use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyTransactionInner {
    pub message: MyMessage,
    pub signatures: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct MyTransaction {
    pub slot: u64,
    pub transaction: MyTransactionInner,
    pub meta: MyMeta,
}

#[derive(Serialize, Deserialize)]
pub struct MyMessage {
    pub header: MyHeader,
    pub account_keys: Vec<[u8; 32]>,
    pub address_table_lookups: Vec<MyMessageAddressTableLookup>,
    pub versioned: bool,
    pub instructions: Vec<MyCompiledInstruction>,
}

#[derive(Serialize, Deserialize)]
pub struct MyHeader {
    pub num_readonly_signed_accounts: u32,
    pub num_readonly_unsigned_accounts: u32,
    pub num_required_signatures: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyCompiledInstruction {
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    pub program_id_index: u32,
}

#[derive(Serialize, Deserialize)]
pub struct MyMessageAddressTableLookup {
    pub account_key: Vec<u8>,
    pub readonly_indexes: Vec<u8>,
    pub writable_indexes: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct MyMeta {
    pub inner_instructions: Vec<MyInnerInstructions>,
    pub loaded_readonly_addresses: Vec<[u8; 32]>,
    pub loaded_writable_addresses: Vec<[u8; 32]>,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub pre_token_balances: Vec<MyTokenBalance>,
    pub post_token_balances: Vec<MyTokenBalance>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyInnerInstructions {
    pub index: u32,
    pub instructions: Vec<MyInnerInstruction>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyInnerInstruction {
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    pub program_id_index: u8,
    pub stack_height: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyTokenBalance {
    pub mint: String,
    pub owner: String,
    pub account_index: u32,
    pub program_id: String,
    pub ui_token_amount: MyUiTokenAmount,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyUiTokenAmount {
    pub ui_amount: f64,
    pub ui_amount_string: String,
    pub decimals: u32,
    pub amount: String,
}

#[derive(Serialize, Deserialize)]
pub struct Swap {
    pub txid: String,
    pub tx_index: u8,
    pub pool: String,
    pub user: String,
    pub mint_in: String,
    pub mint_out: String,
    pub mint_in_reserve: String,
    pub mint_out_reserve: String,
    pub in_decimals: u8,
    pub out_decimals: u8,
    pub amount_in: String,
    pub amount_out: String,
    pub platform: String,
    pub transaction_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionBase {
    pub txid: String,
    pub tx_index: u8,
    pub user: String,
    pub transaction_type: String,
    pub platform: String,
    pub slot: u64,
    pub trading_platform: Option<String>,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PumpAmmDepositEvent {
    pub txid: String,
    pub tx_index: u8,
    pub pool: String,
    pub user: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub pool_base_token_reserves: String,
    pub pool_quote_token_reserves: String,
    pub base_amount_in: String,
    pub quote_amount_in: String,
    pub transaction_type: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PumpAmmWithdrawEvent {
    pub txid: String,
    pub tx_index: u8,
    pub pool: String,
    pub user: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub pool_base_token_reserves: String,
    pub pool_quote_token_reserves: String,
    pub base_amount_out: String,
    pub quote_amount_out: String,
    pub transaction_type: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub struct RaydiumSwap {
    pub pool: String,
    pub mint_in: String,
    pub mint_out: String,
    pub platform: String,
    pub transaction_type: String,
    pub in_decimals: u8,
    pub out_decimals: u8,
    pub mint_in_reserve: u64,
    pub mint_out_reserve: u64,
    pub amount_in: u64,
    pub amount_out: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RaydiumCpmmSwap {
    pub pool: String,
    pub mint_in: String,
    pub mint_out: String,
    pub platform: String,
    pub transaction_type: String,
    pub in_decimals: u8,
    pub out_decimals: u8,
    pub mint_in_reserve: u64,
    pub mint_out_reserve: u64,
    pub amount_in: u64,
    pub amount_out: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PumpAmmSwap {
    pub pool: String,
    pub mint_in: String,
    pub mint_out: String,
    pub mint_in_reserve: u64,
    pub mint_out_reserve: u64,
    pub in_decimals: u8,
    pub out_decimals: u8,
    pub amount_in: u64,
    pub amount_out: u64,
    pub platform: String,
    pub transaction_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PumpAmmPoolCreate {
    pub pool: String,
    pub creator: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub pool_base_token_reserve: u64,
    pub pool_quote_token_reserve: u64,
    pub pool_base_token_account: String,
    pub pool_quote_token_account: String,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub index: u32,
    pub platform: String,
    pub transaction_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct PumpFunSwap {
    pub base_mint: String,
    pub quote_mint: String,
    pub direction: String,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub amount_in: u64,
    pub amount_out: u64,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub transaction_type: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub struct PumpFunPoolCreate {
    pub base_mint: String,
    pub quote_mint: String,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub transaction_type: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub struct RaydiumPoolCreate {
    pub pool: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub base_amount: u64,
    pub quote_amount: u64,
    pub transaction_type: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionEvent {
    pub txid: String,
    pub tx_index: u8,
    pub user: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub direction: String,
    pub reserves: Reserves,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub platform: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub pool: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Reserves {
    pub base: u64,
    pub quote: u64,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StructuredInstruction {
    pub account_key_indexes: Vec<u8>,
    pub program_id_index: u8,
    pub data: Vec<u8>,
    pub inner_instructions: Vec<StructuredInstruction>,
    pub stack_height: u8,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InnerInstruction {
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    pub program_id_index: u8,
    pub stack_height: u8,
    pub inner_instructions: Vec<InnerInstruction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PumpAmmTransaction {
    PoolCreate(PumpAmmPoolCreate),
    Deposit(PumpAmmDepositEvent),
    Withdraw(PumpAmmWithdrawEvent),
    Swap(PumpAmmSwap),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecodedPumpAmmBuyLog {
    pub quote_amount_in: u64,
    pub base_amount_out: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub coin_creator: String,
    pub transaction_type: TransactionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecodedPumpAmmSellLog {
    pub quote_amount_out: u64,
    pub base_amount_in: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub coin_creator: String,
    pub transaction_type: TransactionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
    Buy,
    Sell,
    Deposit,
    Withdraw,
    CreatePool,
}


#[derive(Debug)]
pub struct DecodedPumpAmmCreatePoolEvent {
    pub pool: String,
    pub creator: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub pool_base_token_reserve: u64,
    pub pool_quote_token_reserve: u64,
    pub pool_base_token_account: String,
    pub pool_quote_token_account: String,
    pub index: u16,
    pub event_type: TransactionType,
}

#[derive(Debug)]
pub enum DecodedPumpAmmEvent {
    Swap(DecodedPumpAmmSwapEvent),
    CreatePool(DecodedPumpAmmCreatePoolEvent),
    //Withdraw(DecodedWithdrawEvent),
    //Deposit(DecodedDepositEvent),
}

#[derive(Debug)]
pub struct SwapEventAccounts {
    pub pool: String,
    pub user: String,
    pub base_mint: String,
    pub quote_mint: String,
}

#[derive(Debug)]
pub struct DecodedPumpAmmSwapEvent {
    pub accounts: SwapEventAccounts,
    pub mint_in: String,
    pub mint_out: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub mint_in_reserve: u64,
    pub mint_out_reserve: u64,
    pub event_type: TransactionType,
}

#[derive(Debug)]
pub struct DecodedPumpFunCreatePoolEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub bonding_curve: String,
    pub associated_bonding_curve: String,
    pub event_type: TransactionType
}

#[derive(Debug)]
pub enum DecodedPumpFunEvent {
    Swap(DecodedPumpFunSwapEvent),
    CreatePool(DecodedPumpFunCreatePoolEvent),
    //Withdraw(DecodedWithdrawEvent),
    //Deposit(DecodedDepositEvent),
}

#[derive(Debug)]
pub struct DecodedPumpFunSwapEvent {
    pub accounts: SwapEventAccounts,
    pub mint_in: String,
    pub mint_out: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub mint_in_reserve: u64,
    pub mint_out_reserve: u64,
    pub event_type: TransactionType,
}
#[derive(Debug)]
pub enum DecodedEvent {
    PumpAmm(DecodedPumpAmmEvent),
    PumpFun(DecodedPumpFunEvent)
}

#[derive(Debug)]
pub struct DecodedPumpFunSwapLog {
    pub mint: String,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub user: String,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
}