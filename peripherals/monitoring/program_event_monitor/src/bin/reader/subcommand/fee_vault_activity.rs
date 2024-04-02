use crate::{
    parse::{
        mints::parse_mints,
        time::{datetime_range, parse_datetime},
    },
    CommonArgs,
};
use clap::Parser;
use lollys_lotto::{sdk::constants::ConstMint, PoolRegistry};

#[derive(Debug, Parser)]
pub struct FeeVaultActivity {
    /// Filter to include only data after this timestamp.
    /// Example values include: "2024-01-30", "2024-01-30T13:15:59",
    /// "1 day ago", "5 days ago".
    #[clap(long)]
    since: Option<String>,
    /// Filter to include only data before this timestamp.
    /// Example values include: "2024-01-30", "2024-01-30T13:15:59",
    /// "1 day ago", "5 days ago".
    #[clap(long)]
    until: Option<String>,
    /// Repeatable argument, filter on any number of mints.
    /// If omitted, all mints are searched.
    #[clap(long, short)]
    mint: Vec<String>,
}

impl FeeVaultActivity {
    pub async fn process(self, common_args: CommonArgs) -> anyhow::Result<()> {
        let CommonArgs {
            db,
            mut wtr,
            pool_registry,
            ..
        } = common_args;
        let FeeVaultActivity { since, until, mint } = self;

        let dt_range = datetime_range(parse_datetime(since)?, parse_datetime(until)?)?;
        let mint = if !mint.is_empty() {
            mint
        } else {
            ConstMint::ALL_DEPLOYED
                .iter()
                .map(|m| m.name.to_string())
                .collect()
        };
        let mint_pubkeys = parse_mints(&mint)?;
        let fee_vaults = mint_pubkeys
            .iter()
            .map(|m| (PoolRegistry::fee_vault_address(&pool_registry, m), m))
            .zip(mint)
            .collect::<Vec<_>>();

        println!(
            "fetching fee vault activity\n{}\nfor vaults: {:#?}",
            dt_range, fee_vaults,
        );
        for ((vault, _), name) in fee_vaults {
            let rows = db
                .select_fee_vault_activity_in_datetime_range(&vault, dt_range.clone())
                .await?;
            println!("Got {} rows for mint {}", rows.len(), name,);
            for row in rows {
                wtr.serialize(row)?;
            }
        }
        Ok(())
    }
}
