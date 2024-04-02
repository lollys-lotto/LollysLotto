mod fee_vault_activity;
mod oracle_price_history;
mod pair_activity;
mod swap_activity;
mod vault_activity;

use crate::CommonArgs;
use clap::Parser;
use fee_vault_activity::FeeVaultActivity;
use oracle_price_history::OraclePriceHistory;
use pair_activity::PairActivity;
use swap_activity::SwapActivity;
use vault_activity::VaultActivity;

#[derive(Debug, Parser)]
pub enum Subcommand {
    /// Fetch data from the swap activity table
    SwapActivity(SwapActivity),
    /// Fetch data from the pair activity table
    PairActivity(PairActivity),
    /// Fetch data from the fee vault activity table
    FeeVaultActivity(FeeVaultActivity),
    /// Fetch data from the pool vault activity table
    VaultActivity(VaultActivity),
    /// Fetch historical prices from the oracle price history table
    OraclePriceHistory(OraclePriceHistory),
}

impl Subcommand {
    pub async fn process(self, common_args: CommonArgs) -> anyhow::Result<()> {
        match self {
            Subcommand::SwapActivity(s) => s.process(common_args).await,
            Subcommand::PairActivity(s) => s.process(common_args).await,
            Subcommand::FeeVaultActivity(s) => s.process(common_args).await,
            Subcommand::VaultActivity(s) => s.process(common_args).await,
            Subcommand::OraclePriceHistory(s) => s.process(common_args).await,
        }
    }
}
