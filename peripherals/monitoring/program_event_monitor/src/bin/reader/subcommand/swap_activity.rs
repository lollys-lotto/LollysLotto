use crate::{
    parse::{
        mints::{filter_pairs, parse_mints, parse_pairs},
        time::{datetime_range, parse_datetime},
    },
    CommonArgs,
};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct SwapActivity {
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
    /// Repeatable flag, filter on any number of pairs.
    /// If both this argument and the mint argument are omitted,
    /// all pairs are included.
    #[clap(long, short)]
    pair: Vec<String>,
    /// Repeatable flag, filter on any number of mints.
    /// All pairs including this mint are included.
    #[clap(long, short)]
    mint: Vec<String>,
}

impl SwapActivity {
    pub async fn process(self, common_args: CommonArgs) -> anyhow::Result<()> {
        let CommonArgs {
            db,
            mut wtr,
            pool_registry,
            pool_registry_data,
        } = common_args;
        let SwapActivity {
            since,
            until,
            pair,
            mint,
        } = self;

        let dt_range = datetime_range(parse_datetime(since)?, parse_datetime(until)?)?;
        let pairs = filter_pairs(
            pool_registry_data.all_pairs(&pool_registry),
            parse_pairs(pair, &pool_registry)?,
            parse_mints(&mint)?,
        )?;
        println!(
            "fetching swap activity {} for pairs: {:#?}",
            dt_range, pairs,
        );
        for (pair, name) in pairs {
            let rows = db
                .select_swap_activity_in_datetime_range(&pair, dt_range.clone())
                .await?;
            println!("Got {} rows for pair {}", rows.len(), name,);
            for row in rows {
                wtr.serialize(row)?;
            }
        }
        Ok(())
    }
}
