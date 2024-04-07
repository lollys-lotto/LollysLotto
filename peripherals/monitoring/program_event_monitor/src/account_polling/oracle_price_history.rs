use crate::{account_polling::PolledAccount, prom_metrics::metrics::ORACLE_PRICE};
use log::info;
use lollys_lotto::{AccountHistoryIterator, OraclePriceHistory};
use program_monitor_db::{utils::type_conversions::option_pubkey::Pubkey, Database};
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct OraclePriceHistoryPoll {
    pub address: Pubkey,
    pub poll_every: Duration,
    pub rpc_url: String,
    pub database: Arc<Database>,
}

#[async_trait::async_trait]
impl PolledAccount for OraclePriceHistoryPoll {
    type AccountType = OraclePriceHistory;
    fn rpc_url(&self) -> String {
        self.rpc_url.clone()
    }

    fn address(&self) -> Pubkey {
        self.address
    }

    fn poll_every(&self) -> Duration {
        self.poll_every.clone()
    }

    async fn on_account(self, account: Self::AccountType) -> anyhow::Result<()> {
        let latest_entry = self.database.latest_oracle_price(&self.address).await?;
        if let Some(latest_entry) = latest_entry {
            info!(
                "inserting new oracle prices for {} ({} oracle for mint {}) from slot {} onwards",
                self.address,
                account.oracle_type(),
                account.mint,
                latest_entry.slot,
            );
            let prices = AccountHistoryIterator::from(&account)
                .into_iter()
                .filter_map(|p| {
                    if p.slot > latest_entry.slot {
                        Some(*p)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            self.database
                .insert_oracle_prices(&self.address, &prices)
                .await?;
            if let Some(latest_price) = prices.iter().max_by_key(|p| p.slot) {
                let price_value =
                    latest_price.price.num as f64 / 10_f64.powi(latest_price.price.scale as i32);
                // Set the ORACLE_PRICE metric
                ORACLE_PRICE
                    .with_label_values(&[&account.mint.to_string(), &self.address.to_string()])
                    .set(price_value);
            }
        } else {
            info!(
                "inserting oracle price history and oracle prices for {} ({} oracle for mint {})",
                self.address,
                account.oracle_type(),
                account.mint,
            );
            let info = self
                .database
                .select_oracle_price_history_info(
                    &account.pool_registry,
                    &account.mint,
                    &account.oracle_address,
                )
                .await?;
            if info.is_none() {
                self.database
                    .insert_oracle_price_history_info(
                        &account.pool_registry,
                        &account.oracle_address,
                        account.oracle_type(),
                        &account.mint,
                    )
                    .await?;
            }
            let prices = AccountHistoryIterator::from(&account)
                .into_iter()
                .map(|p| *p)
                .collect::<Vec<_>>();
            self.database
                .insert_oracle_prices(&self.address, &prices)
                .await?;
        }
        info!(
            "finished inserting oracle prices for {} ({} oracle for mint {})",
            self.address,
            account.oracle_type(),
            account.mint,
        );
        Ok(())
    }
}
