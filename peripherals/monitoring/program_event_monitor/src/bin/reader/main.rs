mod parse;
mod subcommand;

use crate::subcommand::Subcommand;
use clap::Parser;
use csv::Writer;
use lollys_lotto::{
    sdk::{constants::MAINNET_POOL_REGISTRY, state::get_pool_registry},
    PoolRegistry,
};
use parse::file::checked_open;
use program_monitor_db::Database;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::{fs::File, sync::Arc};

pub const DEFAULT_MAINNET_URL: &'static str = "https://api.mainnet-beta.solana.com";

/// Query our monitoring DB and write the data out to CSV.
#[derive(Debug, Parser)]
pub struct Opt {
    #[clap(long, env)]
    database_url: String,

    #[clap(long, env, parse(try_from_str=Pubkey::try_from), default_value_t = MAINNET_POOL_REGISTRY)]
    pool_registry: Pubkey,

    #[clap(long, env, default_value = DEFAULT_MAINNET_URL)]
    rpc_url: String,

    /// Path to a CSV file to write fetched data.
    /// By default, this CLI will not overwrite existing files.
    /// Pass the `--overwrite` flag to overwrite existing files.
    #[clap(long)]
    outfile: String,

    /// Overwrite the file specified with `--outfile` if it already exists.
    #[clap(long)]
    overwrite: bool,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let Opt {
        database_url,
        pool_registry,
        rpc_url,
        outfile,
        overwrite,
        subcommand,
    } = Opt::parse();
    let db = Arc::new(Database::new(&database_url, None).await?);

    let wtr = checked_open(outfile, overwrite)?;

    let client = RpcClient::new(rpc_url);
    let pool_registry_data = get_pool_registry(&pool_registry, &client).await?;

    let common_args = CommonArgs {
        db,
        wtr,
        pool_registry,
        pool_registry_data,
    };
    subcommand.process(common_args).await
}

pub struct CommonArgs {
    pub db: Arc<Database>,
    pub wtr: Writer<File>,
    pub pool_registry: Pubkey,
    pub pool_registry_data: PoolRegistry,
}
