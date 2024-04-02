use anyhow::anyhow;
use clap::Parser;
use lollys_lotto::{sdk::constants::MAINNET_POOL_REGISTRY, EventEmitter};
use lollys_lotto_event_monitor::event_backfilling::{
    backfiller, backfiller::SSLv2Backfiller, ProgramEventBackfiller,
};
use program_monitor_db::Database;
use solana_sdk::pubkey::Pubkey;
use std::{sync::Arc, time::Duration};

const DEFAULT_SLEEP_BETWEEN_WINDOWS: Duration = Duration::from_secs(5);

/// SSLv2 Program Event Backfiller.
///
/// This is a standalone binary for backfilling our historical database
/// with any log-emitted program events that it may not already contain.
#[derive(Debug, Parser)]
pub struct Opt {
    /// Must be a credentialed URL.
    #[clap(long, env)]
    database_url: String,

    #[clap(long, env)]
    rpc_url: String,

    /// Pubkey of the target pool registry, defaults to F451mjRqGEu1azbj46v4FuMEt1CacaPHQKUHzuTqKp4R
    #[clap(long, env, parse(try_from_str=Pubkey::try_from), default_value_t = MAINNET_POOL_REGISTRY)]
    pool_registry: Pubkey,

    /// Lower number. If left out, the backfiller will search
    /// back until the event with seq-num zero. If provided,
    /// the database must contain this event, since the backfiller must
    /// look up its transaction signature.
    #[clap(long, env)]
    start_seq_num: Option<i64>,

    /// Higher number. If provided, the database must contain this
    /// event.
    #[clap(long, env)]
    end_seq_num: i64,

    /// Print gaps between start_seq_num and end_seq_num, and exit.
    #[clap(long, env)]
    show_gaps: bool,

    /// Backfill failed transactions in addition to successful ones.
    #[clap(long, env)]
    include_failed_transactions: bool,

    /// Throttle the number of transactions fetched and searched before
    /// cooldown periods.
    #[clap(long, default_value_t = 1000)]
    rpc_sig_limit: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let opt = Opt::parse();

    if opt.show_gaps {
        let database = Arc::new(Database::new(&opt.database_url, None).await?);
        if opt.start_seq_num.is_none() {
            return Err(anyhow!(
                "Must provide a --start-seq-num to show gaps in range"
            ));
        }

        for (more_recent, less_recent) in
            backfiller::get_event_gaps(database, opt.start_seq_num.unwrap(), opt.end_seq_num)
                .await?
        {
            let gap_size = more_recent.seq_num - less_recent.seq_num - 1;
            println!(
                "{} missing event(s) between {} and {}",
                gap_size, less_recent.seq_num, more_recent.seq_num
            );
        }
        return Ok(());
    }

    SSLv2Backfiller::new(
        &opt.database_url,
        &opt.rpc_url,
        opt.pool_registry,
        DEFAULT_SLEEP_BETWEEN_WINDOWS,
        opt.rpc_sig_limit,
        opt.start_seq_num,
        opt.end_seq_num,
    )
    .await?
    .backfill(EventEmitter::address(), !opt.include_failed_transactions)
    .await
}
