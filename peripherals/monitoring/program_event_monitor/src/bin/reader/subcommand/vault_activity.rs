use crate::{
    parse::{
        mints::{parse_mints, parse_secondary_vaults, resolve_vaults},
        time::{datetime_range, parse_datetime},
    },
    CommonArgs,
};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct VaultActivity {
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
    /// Repeatable argument to specify all vaults, including
    /// both the pool's main vault and all of its secondary vaults.
    /// e.g. "--pool SOL" will filter on
    /// SOL's main vault, SOL's USDC vault, SOL's MSOL vault, etc.
    #[clap(long, short)]
    pool: Vec<String>,
    /// Repeatable argument to specify main vaults for a specific token
    #[clap(long, short)]
    main_vault: Vec<String>,
    /// Repeatable argument to specify secondary vaults containing a specific token
    #[clap(long, short)]
    secondary_vault: Vec<String>,
}

impl VaultActivity {
    pub async fn process(self, common_args: CommonArgs) -> anyhow::Result<()> {
        let CommonArgs {
            db,
            mut wtr,
            pool_registry,
            pool_registry_data,
        } = common_args;
        let VaultActivity {
            since,
            until,
            pool,
            main_vault,
            secondary_vault,
        } = self;

        let ssl_pools = parse_mints(&pool)?;
        let main_vaults = parse_mints(&main_vault)?;
        let secondary_vaults = parse_secondary_vaults(&secondary_vault)?;
        let vaults = resolve_vaults(
            &ssl_pools,
            &main_vaults,
            &secondary_vaults,
            &pool_registry,
            pool_registry_data,
        );

        let dt_range = datetime_range(parse_datetime(since)?, parse_datetime(until)?)?;

        println!(
            "fetching pool vault activity\n{}\nfor vaults: {:#?}",
            dt_range, vaults,
        );
        for (vault, name) in vaults {
            let rows = db
                .select_pool_vault_activity_in_datetime_range(&vault, dt_range.clone())
                .await?;
            println!("Got {} rows for mint {}", rows.len(), name,);
            for row in rows {
                wtr.serialize(row)?;
            }
        }
        Ok(())
    }
}
