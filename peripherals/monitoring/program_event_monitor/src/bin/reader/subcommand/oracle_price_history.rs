use crate::{
    parse::{mints::parse_mints, time::slot_range},
    CommonArgs,
};
use clap::Parser;
use lollys_lotto::sdk::constants::ConstMint;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
pub struct OraclePriceHistory {
    /// Filter to include only data after this slot.
    #[clap(long)]
    since: Option<u64>,
    /// Filter to include only data before this slot.
    #[clap(long)]
    until: Option<u64>,
    /// Repeatable argument, filter on any number of mints.
    /// If omitted, all mints are searched.
    mint: Vec<String>,
}

impl OraclePriceHistory {
    pub async fn process(self, common_args: CommonArgs) -> anyhow::Result<()> {
        let CommonArgs {
            db,
            mut wtr,
            pool_registry_data,
            ..
        } = common_args;
        let OraclePriceHistory { since, until, mint } = self;

        let slot_range = slot_range(since, until)?;
        let mint = if !mint.is_empty() {
            mint
        } else {
            ConstMint::ALL_DEPLOYED
                .iter()
                .map(|m| m.name.to_string())
                .collect()
        };
        let mint_pubkeys = parse_mints(&mint)?;
        let price_histories = mint_pubkeys
            .iter()
            .map(|m| {
                pool_registry_data
                    .find_pool(*m)
                    .unwrap()
                    .oracle_price_histories[0]
            })
            .zip(mint)
            .collect::<Vec<_>>();

        println!(
            "fetching historical prices \n{}\nfor oracles: {:#?}",
            slot_range, price_histories,
        );
        for (price_history, name) in price_histories {
            let rows = db
                .select_oracle_prices_in_slot_range(&price_history, slot_range.clone())
                .await?
                .into_iter()
                .map(|p| UiHistoricalPrice {
                    price: p.price.into(),
                    slot: p.slot,
                })
                .collect::<Vec<_>>();
            println!("Got {} rows for mint {}", rows.len(), name,);
            for row in rows {
                wtr.serialize(row)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiHistoricalPrice {
    pub price: Decimal,
    pub slot: u64,
}
