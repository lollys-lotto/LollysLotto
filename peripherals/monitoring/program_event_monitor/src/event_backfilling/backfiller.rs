use crate::{
    address_calculator::AddressCalculator,
    event_backfilling::{MissingEventWindow, ProgramEventBackfiller},
    event_processing::process_event,
    log_parsing::LoggedTransactionFailure,
};
use anchor_lang::solana_program::clock::Slot;
use anyhow::anyhow;
use log::{error, info};
use lollys_lotto::SSLProgramEvent;
use program_monitor_db::{
    utils::type_conversions::option_pubkey::Pubkey, Database, ProgramEventRow,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{signature::Signature, transaction::TransactionError};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};

/// Searches for missing SSLv2 event data in a range of `seq_num`. This will only work
/// for ranges where the start and end of the search bound are already indexed.
#[derive(Clone)]
pub struct SSLv2Backfiller {
    pub database: Arc<Database>,
    pub rpc_client: Arc<RpcClient>,
    pub address_calculator: Arc<RwLock<AddressCalculator>>,
    pub sleep_between_windows: Duration,
    pub signature_fetch_limit: usize,
    pub start_seq_num: Option<i64>,
    pub end_seq_num: i64,
}

impl SSLv2Backfiller {
    pub async fn new(
        database_url: &str,
        rpc_url: &str,
        pool_registry: Pubkey,
        sleep_between_windows: Duration,
        signature_fetch_limit: usize,
        start_seq_num: Option<i64>,
        end_seq_num: i64,
    ) -> anyhow::Result<Self> {
        if let Some(start_seq_num) = start_seq_num {
            if start_seq_num >= end_seq_num {
                return Err(anyhow!(
                    "Invalid seq num range, start must be < end ({} < {})",
                    start_seq_num,
                    end_seq_num
                ));
            }
        }
        Ok(Self {
            database: Arc::new(Database::new(database_url, None).await?),
            address_calculator: Arc::new(RwLock::new(
                AddressCalculator::new(RpcClient::new(rpc_url.to_string()), pool_registry).await?,
            )),
            rpc_client: Arc::new(RpcClient::new(rpc_url.to_string())),
            sleep_between_windows,
            signature_fetch_limit,
            start_seq_num,
            end_seq_num,
        })
    }
}

pub async fn get_event_gaps(
    db: Arc<Database>,
    start_seq_num: i64,
    end_seq_num: i64,
) -> anyhow::Result<Vec<(ProgramEventRow, ProgramEventRow)>> {
    let mut rows = db
        .fetch_events_by_seq_num_range(start_seq_num, end_seq_num)
        .await
        .map_err(|e| {
            anyhow!(
                "Failed to fetch events from database between range {} - {}: {:?}",
                start_seq_num,
                end_seq_num,
                e
            )
        })?;

    if rows.is_empty() {
        return Err(anyhow!(
            "Could not find a seq_num for either {} or {}",
            start_seq_num,
            end_seq_num
        ));
    }

    // Covers an edge case where all events inside the specified range are not indexed,
    // except for either `self.start_seq_num` or `self.end_seq_num`.
    if rows.len() == 1 {
        let only_found = rows[0].seq_num;
        let not_found = if only_found == start_seq_num {
            end_seq_num
        } else {
            start_seq_num
        };
        return Err(anyhow!(
            "Only found an event for seq_num {} but not seq_num {}",
            only_found,
            not_found
        ));
    }

    // Sort in descending order
    rows.sort_by_key(|event| std::cmp::Reverse(event.seq_num));

    Ok(rows
        .windows(2)
        .filter_map(|window| {
            if let [more_recent, less_recent] = window {
                if more_recent.seq_num - less_recent.seq_num > 1 {
                    Some((more_recent.clone(), less_recent.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .rev()
        .collect())
}

#[async_trait::async_trait]
impl ProgramEventBackfiller for SSLv2Backfiller {
    type Event = SSLProgramEvent;
    type Error = anyhow::Error;

    fn target_program() -> Pubkey {
        lollys_lotto::ID
    }

    fn rpc_client(&self) -> Arc<RpcClient> {
        self.rpc_client.clone()
    }

    fn max_transaction_batch_size(&self) -> usize {
        self.signature_fetch_limit
    }

    async fn between_missing_event_windows(&mut self) -> anyhow::Result<(), Self::Error> {
        sleep(self.sleep_between_windows).await;
        Ok(())
    }

    /// This function fetches all the unique successful events and sorts them to find missing event intervals.
    /// we will save the missing interval details in MissingEventInfo
    async fn get_missing_event_windows(&self) -> anyhow::Result<Vec<MissingEventWindow>> {
        if self.start_seq_num.is_none() {
            let row = self
                .database
                .select_event_by_seq_num(self.end_seq_num)
                .await
                .map_err(|e| {
                    anyhow!(
                        "Could not find an event with seq_num {}: {}",
                        self.end_seq_num,
                        e
                    )
                })?;
            // The database doesn't enforce that there should only be one of these rows.
            let row = row.first().unwrap();
            info!(
                "Searching for all events before {} at transaction {}",
                self.end_seq_num, row.transaction_signature
            );
            let window = MissingEventWindow {
                least_recent: None,
                most_recent: Some(row.transaction_signature),
                stop_after_num_found: None,
            };
            return Ok(vec![window]);
        }

        let missing_events: Vec<_> = get_event_gaps(
            self.database.clone(),
            self.start_seq_num.unwrap(),
            self.end_seq_num,
        )
        .await?
        .into_iter()
        .map(|(more_recent, less_recent)| {
            let stop_after_num_found = more_recent.seq_num - less_recent.seq_num - 1;
            let window = MissingEventWindow {
                least_recent: Some(less_recent.transaction_signature),
                most_recent: Some(more_recent.transaction_signature),
                stop_after_num_found: Some(stop_after_num_found as usize),
            };
            info!("Found missing event window: {:?}", window);
            window
        })
        .collect();

        info!("Found {} missing event windows", missing_events.len());
        Ok(missing_events)
    }

    async fn on_event(
        &mut self,
        signature: Signature,
        slot: Slot,
        execution_error: Option<LoggedTransactionFailure>,
        rpc_error_response: Option<TransactionError>,
        raw_event: String,
        event: SSLProgramEvent,
    ) -> anyhow::Result<bool> {
        let is_not_error = execution_error.is_none();
        match process_event(
            self.database.clone(),
            self.address_calculator.clone(),
            signature,
            slot,
            execution_error,
            rpc_error_response,
            raw_event,
            event,
            true,
        )
        .await
        {
            Ok(is_not_duplicate) => Ok(is_not_duplicate && is_not_error),
            Err(e) => {
                error!("uncaught error in event processing: {:?}", e);
                Ok(false)
            }
        }
    }
}
