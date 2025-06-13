use {
    backoff::{ExponentialBackoff, future::retry},
    clap::Parser as ClapParser,
    futures::{future::TryFutureExt, sink::SinkExt, stream::StreamExt},
    log::{error, info},
    parser_core::TransactionParserNew,
    serde::{Deserialize, Serialize},
    serde_json,
    std::{collections::HashMap, env, sync::Arc, time::Duration},
    tokio::sync::Mutex,
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::{GeyserGrpcClient, Interceptor},
    yellowstone_grpc_proto::{
        geyser::SubscribeRequestFilterTransactions,
        prelude::{
            CommitmentLevel, SubscribeRequest, SubscribeRequestPing, SubscribeUpdateTransaction,
            subscribe_update::UpdateOneof,
        },
    },
};

type TxnFilterMap = HashMap<String, SubscribeRequestFilterTransactions>;

const PUMP_AMM_PROGRAM_ID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
const PUMP_FUN_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
const RAYDIUM_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const ADDRESS_TO_STREAM: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";

#[derive(Debug, Clone, ClapParser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, help = "gRPC endpoint")]
    endpoint: String,

    #[clap(long, help = "X-Token")]
    x_token: String,
}

impl Args {
    async fn connect(&self) -> anyhow::Result<GeyserGrpcClient<impl Interceptor>> {
        println!("Connecting to {}", self.endpoint);
        GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .x_token(Some(self.x_token.clone()))?
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .max_decoding_message_size(1024 * 1024 * 1024)
            .connect()
            .await
            .map_err(Into::into)
    }

    pub fn get_txn_updates(&self) -> SubscribeRequest {
        let mut transactions: TxnFilterMap = TxnFilterMap::new();

        transactions.insert(
            "client".to_owned(),
            SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                account_include: vec![
                    PUMP_AMM_PROGRAM_ID.to_string(),
                    PUMP_FUN_PROGRAM_ID.to_string(),
                    //RAYDIUM_PROGRAM_ID.to_string(),
                ],
                account_exclude: vec![],
                account_required: vec![],
                signature: None,
            },
        );

        SubscribeRequest {
            accounts: HashMap::default(),
            slots: HashMap::default(),
            transactions,
            transactions_status: HashMap::default(),
            blocks: HashMap::default(),
            blocks_meta: HashMap::default(),
            entry: HashMap::default(),
            commitment: Some(CommitmentLevel::Processed as i32),
            accounts_data_slice: Vec::default(),
            ping: None,
            from_slot: None,
        }
    }
}

struct State {
    last_slot: Option<u64>,
    attempts_since_success: u32,
}

const MAX_RETRY_WITH_FROM_SLOT: u32 = 5;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    unsafe {
        env::set_var(
            env_logger::DEFAULT_FILTER_ENV,
            env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
        );
    }
    env_logger::init();

    let args = Args::parse();
    let state = Arc::new(Mutex::new(State {
        last_slot: None,
        attempts_since_success: 0,
    }));

    let parser = TransactionParserNew::new();

    // The default exponential backoff strategy intervals:
    // [500ms, 750ms, 1.125s, 1.6875s, 2.53125s, 3.796875s, 5.6953125s,
    // 8.5s, 12.8s, 19.2s, 28.8s, 43.2s, 64.8s, 97s, ... ]
    retry(ExponentialBackoff::default(), move || {
        let args = args.clone();
        let state = state.clone();
        let parser = parser.clone();

        async move {
            // decide whether to send `from_slot` this time
            let (maybe_slot, use_from_slot) = {
                let st = state.lock().await;
                (st.last_slot, st.attempts_since_success < MAX_RETRY_WITH_FROM_SLOT)
            };

            if let Some(slot) = maybe_slot {
                if use_from_slot {
                    info!("→ resuming from slot {}, current attempt {}", slot, state.lock().await.attempts_since_success);
                } else {
                    info!(
                        "→ subscribing without 'from_slot' (dropped 'from_slot' after {} fails, current attempt {})",
                        MAX_RETRY_WITH_FROM_SLOT, state.lock().await.attempts_since_success
                    );
                }
            }

            let client = match args.connect().await {
                Ok(c) => c,
                Err(e) => {
                    // count this failure
                    let mut st = state.lock().await;
                    st.attempts_since_success += 1;
                    return Err::<(), backoff::Error<anyhow::Error>>(backoff::Error::transient(e));
                }
            };
            info!("Connected");

            let mut request = args.get_txn_updates();
            if let Some(slot) = maybe_slot {
                if use_from_slot {
                    request.from_slot = Some(slot);
                }
            }

            geyser_subscribe(client, request, state.clone(), parser)
                .await
                .map_err(backoff::Error::transient)?;

            // we never return Ok: every disconnect is an Err to keep retrying
            unreachable!("subscribe never returns Ok");
        }
            .inspect_err(|error| error!("retry error: {}", error))
    })
        .await
        .unwrap();
    Ok(())
}

/// Handles one subscribe‐session.  
/// On first message: resets `attempts_since_success = 0`.  
/// On each txn: updates `last_slot`.  
/// On any error or clean close: if we never saw a message, increments `attempts_since_success` and returns Err.
async fn geyser_subscribe(
    mut client: GeyserGrpcClient<impl Interceptor>,
    request: SubscribeRequest,
    state: Arc<Mutex<State>>,
    parser: TransactionParserNew,
) -> anyhow::Result<()> {
    let (mut subscribe_tx, mut stream) = client.subscribe_with_request(Some(request)).await?;

    info!("stream opened");

    let mut seen_any = false;

    while let Some(frame) = stream.next().await {
        match frame {
            Ok(msg) => {
                // first message → reset failure count
                if !seen_any {
                    seen_any = true;
                    let mut st = state.lock().await;
                    st.attempts_since_success = 0;
                }

                match msg.update_oneof {
                    Some(UpdateOneof::Transaction(upd)) => {
                        // update our last_slot
                        let slot = upd.slot;
                        {
                            let mut st = state.lock().await;
                            st.last_slot = Some(slot);
                        }

                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis();

                        // Clone upd first to avoid partial move issues
                        let upd_clone = upd.clone();

                        let txn = match upd.transaction {
                            Some(txn) => txn,
                            None => {
                                error!("Transaction update was empty");
                                continue;
                            }
                        };

                        let raw_signature = txn.signature.clone();
                        info!(
                            "signature: {}, slot: {}, timestamp: {},",
                            bs58::encode(&raw_signature).into_string(),
                            slot,
                            timestamp,
                        );

                        let raw_transaction = txn.clone().transaction.expect("transaction empty");
                        let decoded_txn = parser.decode_transaction(&upd_clone);
                        if decoded_txn.is_empty() && has_balance_change(&upd_clone) {
                            println!(
                                "https://solscan.io/tx/{}",
                                bs58::encode(&raw_signature).into_string()
                            );
                        }
                        let raw_message = raw_transaction.message.expect("message empty").clone();
                        let _header = raw_message.header.expect("header empty");
                        let _meta = txn.meta.expect("Meta empty");

                        // You can continue processing from here if needed later
                    }
                    Some(UpdateOneof::Ping(_)) => {
                        subscribe_tx
                            .send(SubscribeRequest {
                                ping: Some(SubscribeRequestPing { id: 1 }),
                                ..Default::default()
                            })
                            .await?;
                    }
                    Some(UpdateOneof::Pong(_)) => {}
                    None => {
                        error!("update not found in the message");
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                error!("stream error: {:?}", e);
                // if we never got a message, count this as a failure
                if !seen_any {
                    let mut st = state.lock().await;
                    st.attempts_since_success += 1;
                }
                return Err(e.into());
            }
        }
    }

    info!("stream closed");
    if !seen_any {
        let mut st = state.lock().await;
        st.attempts_since_success += 1;
    }
    Ok(())
}

pub fn has_balance_change(transaction: &SubscribeUpdateTransaction) -> bool {
    let meta = transaction.clone().transaction.unwrap().meta.unwrap();
    let pre_token_balances = meta.pre_token_balances;
    let post_token_balances = meta.post_token_balances;
    let mut has_balance_c = false;
    for balance in pre_token_balances.clone() {
        let mint = balance.mint;
        let owner = balance.owner;
        let post_balance = post_token_balances
            .iter()
            .find(|post_balance| post_balance.mint == mint && post_balance.owner == owner);
        match post_balance {
            Some(post_balance) => {
                let pre_amount = balance.ui_token_amount.unwrap().ui_amount;
                let post_amount = post_balance.clone().ui_token_amount.unwrap().ui_amount;
                if pre_amount != post_amount {
                    has_balance_c = true;
                }
            }
            None => {}
        }
    }
    for balance in post_token_balances {
        let mint = balance.mint;
        let owner = balance.owner;
        let pre_balance = pre_token_balances
            .iter()
            .find(|pre_balance| pre_balance.mint == mint && pre_balance.owner == owner);
        match pre_balance {
            Some(pre_balance) => {
                let post_amount = balance.ui_token_amount.unwrap().ui_amount;
                let pre_amount = pre_balance.clone().ui_token_amount.unwrap().ui_amount;
                if pre_amount != post_amount {
                    has_balance_c = true;
                }
            }
            None => {}
        }
    }
    has_balance_c
}
