use anchor_lang::solana_program::clock::Slot;
use clap::Parser;
use lollys_lotto::{sdk::constants::MAINNET_POOL_REGISTRY, EventEmitter, SSLProgramEvent};
use lollys_lotto_event_monitor::{
    account_polling::{oracle_price_history::OraclePriceHistoryPoll, PolledAccount},
    address_calculator::AddressCalculator,
    event_backfilling::{backfiller::SSLv2Backfiller, ProgramEventBackfiller},
    event_log_sub::ProgramEventLogSubscriber,
    event_processing::process_event,
    log_parsing::LoggedTransactionFailure,
    prom_metrics::metrics::{self, SSL_METRICS_REGISTRY},
};
use log::{debug, error, info};
use program_monitor_db::Database;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signature, transaction::TransactionError};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, task::JoinHandle, time::sleep};

const DEFAULT_SLEEP_BETWEEN_RECONNECTS: Duration = Duration::from_secs(5);

#[derive(Debug, Parser)]
pub struct Opt {
    #[clap(long, env)]
    database_url: String,

    #[clap(long, env)]
    http_url: String,

    #[clap(long, env)]
    ws_url: String,

    #[clap(long, env)]
    pg_ssl_cert: Option<String>,

    #[clap(long, env)]
    include_failed_transactions: bool,

    #[clap(long, env)]
    poll_accounts: bool,

    #[clap(long, env, default_value_t = 100)]
    backfill_every: u32,

    #[clap(long, env, parse(try_from_str=Pubkey::try_from), default_value_t = MAINNET_POOL_REGISTRY)]
    pool_registry: Pubkey,

    #[clap(long, env, default_value_t = 9090)]
    metrics_port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let opt = Opt::parse();

    let ssl_metrics_registry = SSL_METRICS_REGISTRY.clone();
    // Start the metrics server in a separate asynchronous task
    let metrics_server_task = tokio::spawn(async move {
        info!("Starting metrics server on port {}", opt.metrics_port);
        metrics::serve(ssl_metrics_registry, opt.metrics_port).await;
    });

    // Monitor program logs for events
    let event_monitor = SSLv2Monitor::new(
        &opt.database_url,
        &opt.http_url,
        &opt.ws_url,
        opt.pg_ssl_cert.as_ref().map(|s| s.as_str()),
        opt.pool_registry,
        None,
        opt.include_failed_transactions,
        opt.backfill_every as i64,
    )
    .await?;

    // Poll all oracle price history accounts
    let pool_registry = event_monitor
        .address_calculator
        .read()
        .await
        .pool_registry_data;

    // Start and join all tasks.
    let mut all_tasks = vec![];

    if opt.poll_accounts {
        for idx in 0..pool_registry.num_entries {
            pool_registry.entries[idx as usize]
                .oracle_price_histories
                .iter()
                .filter(|addr| **addr != Pubkey::default())
                .for_each(|p| {
                    all_tasks.push(
                        OraclePriceHistoryPoll {
                            address: *p,
                            poll_every: Duration::from_secs(60 * 7),
                            rpc_url: opt.http_url.clone(),
                            database: event_monitor.database.clone(),
                        }
                        .poll_account(),
                    );
                });
        }
    }

    let join_handle = event_monitor.watch_for_events();
    all_tasks.push(join_handle);
    all_tasks.push(metrics_server_task);
    futures::future::join_all(all_tasks).await;
    Ok(())
}

// TODO Track connection status and rate of data ingress,
//   to make sure we are consistently indexing things, allowing us to submit error reports
/// Watches for Events emitted on SSLv2,
/// saves their data to Postgres.
#[derive(Clone)]
pub struct SSLv2Monitor {
    database: Arc<Database>,
    ws_url: String,
    http_url: String,
    address_calculator: Arc<RwLock<AddressCalculator>>,
    sleep_between_reconnects: Duration,
    backfiller_start: Arc<RwLock<Option<i64>>>,
    backfiller_end: Arc<RwLock<Option<i64>>>,
    backfiller_task: Arc<RwLock<Option<JoinHandle<anyhow::Result<()>>>>>,
    backfill_every_n_events: i64,
    include_failed_transactions: bool,
}

impl SSLv2Monitor {
    pub async fn new(
        database_url: &str,
        http_url: &str,
        ws_url: &str,
        pg_ssl_cert: Option<&str>,
        pool_registry: Pubkey,
        sleep_between_reconnects: Option<Duration>,
        include_failed_transactions: bool,
        backfill_every_n: i64,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            database: Arc::new(Database::new(database_url, pg_ssl_cert).await?),
            ws_url: ws_url.to_string(),
            http_url: http_url.to_string(),
            address_calculator: Arc::new(RwLock::new(
                AddressCalculator::new(RpcClient::new(http_url.to_string()), pool_registry).await?,
            )),
            sleep_between_reconnects: sleep_between_reconnects
                .unwrap_or(DEFAULT_SLEEP_BETWEEN_RECONNECTS),

            backfill_every_n_events: backfill_every_n,
            backfiller_start: Arc::new(RwLock::new(None)),
            backfiller_end: Arc::new(RwLock::new(None)),
            backfiller_task: Arc::new(RwLock::new(None)),
            include_failed_transactions,
        })
    }

    pub async fn backfiller(&self, start_seq_num: i64, end_seq_num: i64) -> SSLv2Backfiller {
        SSLv2Backfiller {
            database: self.database.clone(),
            rpc_client: Arc::new(RpcClient::new(self.http_url.clone())),
            address_calculator: self.address_calculator.clone(),
            sleep_between_windows: self.sleep_between_reconnects.clone(),
            signature_fetch_limit: 1000,
            start_seq_num: Some(start_seq_num),
            end_seq_num,
        }
    }
}

#[async_trait::async_trait]
impl ProgramEventLogSubscriber for SSLv2Monitor {
    type Event = SSLProgramEvent;
    fn target_program() -> Pubkey {
        lollys_lotto::ID
    }

    fn ws_url(&self) -> String {
        self.ws_url.clone()
    }

    async fn on_reconnect(&self) -> anyhow::Result<()> {
        sleep(self.sleep_between_reconnects).await;
        Ok(())
    }

    async fn on_event(
        mut self,
        signature: Signature,
        slot: Slot,
        execution_error: Option<LoggedTransactionFailure>,
        rpc_error_response: Option<TransactionError>,
        raw_event: String,
        event: SSLProgramEvent,
    ) {
        let event_id = event.event_id();
        if !self.include_failed_transactions && execution_error.is_some() {
            debug!(
                "skipping failed transaction {} with error: {:?}",
                signature,
                execution_error.unwrap()
            );
            return;
        }

        if let Err(e) = process_event(
            self.database.clone(),
            self.address_calculator.clone(),
            signature,
            slot,
            execution_error,
            rpc_error_response,
            raw_event,
            event,
            false,
        )
        .await
        {
            error!("uncaught error in event processing: {:?}", e);
        }

        {
            let start_seq_num_clone = self.backfiller_start.clone();
            let mut start_seq_num = self.backfiller_start.write().await;
            let mut end_seq_num = self.backfiller_end.write().await;
            let mut task = self.backfiller_task.write().await;
            if task.as_ref().map_or(true, |task| task.is_finished())
                && start_seq_num.map_or(false, |seq_num| {
                    seq_num + self.backfill_every_n_events < event_id
                })
            {
                *end_seq_num = Some(event_id);
                let mut backfiller = self
                    .backfiller(start_seq_num.unwrap(), end_seq_num.unwrap())
                    .await;
                info!("spawning backfiller routine");
                *task = Some(tokio::spawn(async move {
                    if let Err(e) = backfiller.backfill(EventEmitter::address(), true).await {
                        error!("backfiller task failed unexpectedly: {e}");
                    };
                    let mut start_seq_num = start_seq_num_clone.write().await;
                    *start_seq_num = Some(backfiller.end_seq_num);

                    info!("current backfiller routine completed successfully");
                    Ok(())
                }));
            } else if start_seq_num.is_none() {
                info!("Setting start_seq_num to {}", event_id);
                *start_seq_num = Some(event_id);
            }
        }
    }
}
