pub mod backfiller;

use crate::log_parsing::{
    check_for_program_error, parse_transaction_logs, LoggedTransactionFailure,
};
use anchor_lang;
use log::{debug, error, info, warn};
use solana_client::{
    client_error::{ClientError, ClientErrorKind},
    nonblocking::rpc_client::RpcClient,
    rpc_client::GetConfirmedSignaturesForAddress2Config,
};
use solana_sdk::{
    clock::Slot, commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature,
    transaction::TransactionError,
};
use std::{fmt::Display, sync::Arc};

/// A search space where there is a gap in event data.
/// This search space is bounded by transaction signatures.
/// It is searched backward in time, i.e. from the most recent signature `at_or_before`,
/// backwards in time up to `back_until`. The specified signatures are included
/// in the search space.
#[derive(Debug, Clone)]
pub struct MissingEventWindow {
    /// The lower bound of the search space. How far in the past to search.
    pub least_recent: Option<Signature>,
    /// The upper bound of the search space.
    pub most_recent: Option<Signature>,
    /// Optionally terminate the search early if some number of
    /// events are found.
    pub stop_after_num_found: Option<usize>,
}

/// This trait provides a pre-defined search routine for recovering data
/// associated with log-emitted Anchor events which coincide with mutations
/// of account data on a target account.
#[async_trait::async_trait]
pub trait ProgramEventBackfiller: Send + Sync + 'static {
    /// The Anchor event type.
    type Event: anchor_lang::Event + anchor_lang::AnchorDeserialize + Send;

    /// This trait makes RPC calls that can fail, so your error type must
    /// therefore be able to be converted from a [solana_client::client_error::ClientError].
    type Error: From<ClientError> + Display;

    /// The target program that emits events. This is needed
    /// to properly parse program logs for emitted events.
    fn target_program() -> Pubkey;

    fn rpc_client(&self) -> Arc<RpcClient>;

    /// The search over transactions occurs in batches. This sets a limit
    /// on the batch size.
    fn max_transaction_batch_size(&self) -> usize {
        1000
    }

    /// After searching each event window,
    /// optionally sleep or produce other log information, etc.
    async fn between_missing_event_windows(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Define how to procure a collection of gaps in data.
    /// Each of these gaps will be searched for missing events
    /// and processed with [Self::on_event].
    async fn get_missing_event_windows(&self) -> Result<Vec<MissingEventWindow>, Self::Error>;

    /// Define what to do with each new incoming event.
    async fn on_event(
        &mut self,
        signature: Signature,
        slot: Slot,
        execution_error: Option<LoggedTransactionFailure>,
        rpc_error_response: Option<TransactionError>,
        raw_event: String,
        event: Self::Event,
    ) -> Result<bool, Self::Error>;

    /// The top-level execution for the search routine defined on this trait.
    async fn backfill(
        &mut self,
        target_account: Pubkey,
        skip_failed_transactions: bool,
    ) -> Result<(), Self::Error> {
        let missing_events = self.get_missing_event_windows().await?;

        for window in missing_events {
            if let Err(e) = self
                .search_missing_event_window(target_account, &window, skip_failed_transactions)
                .await
            {
                error!(
                    "Search of missing event window terminated unexpectedly {:?}: {}",
                    window, e
                );
                return Err(e);
            }
            self.between_missing_event_windows().await?;
        }

        Ok(())
    }

    /// Fetch and parse any existing transaction logs for instances of `Self::Event`,
    /// and for each event, call [Self::on_event].
    async fn parse_transaction_logs_for_missing_events(
        &mut self,
        signature: Signature,
    ) -> Result<usize, Self::Error> {
        let mut num_recovered = 0usize;

        let transaction = self
            .rpc_client()
            .get_transaction_with_config(
                &signature,
                solana_client::rpc_config::RpcTransactionConfig {
                    encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
                    commitment: None,
                    max_supported_transaction_version: Some(0),
                },
            )
            .await
            .map_err(|e| {
                error!(
                    "Error fetching transaction logs for signature {}: {:?}",
                    signature, e
                );
                e.into()
            })?;

        let rpc_error_response = transaction.transaction.meta.clone().unwrap().err;

        let transaction_logs = match transaction.transaction.meta.unwrap().log_messages {
            solana_transaction_status::option_serializer::OptionSerializer::Some(logs) => logs,
            _ => {
                warn!("No transaction logs found for signature {}", signature);
                return Ok(0);
            }
        };

        let execution_error = transaction_logs
            .last()
            .map(|l| check_for_program_error(l))
            .flatten()
            .or(rpc_error_response
                .clone()
                .map(|e| LoggedTransactionFailure::from(e)));

        let events = parse_transaction_logs(transaction_logs, &Self::target_program().to_string());

        for (raw_event, event) in events {
            let should_count_as_missing = self
                .on_event(
                    signature,
                    transaction.slot,
                    execution_error.clone(),
                    rpc_error_response.clone(),
                    raw_event,
                    event,
                )
                .await?;

            if should_count_as_missing {
                num_recovered += 1;
            }
        }

        Ok(num_recovered)
    }

    /// Iterately search through a single [MissingEventWindow], targeting a specific
    /// account for data mutations.
    async fn search_missing_event_window(
        &mut self,
        target_account: Pubkey,
        window: &MissingEventWindow,
        skip_failed_transactions: bool,
    ) -> Result<(), Self::Error> {
        let until = window.least_recent;
        let mut before = window.most_recent;
        let mut recovered_events = 0usize;

        // Process the initial 'before' signature
        if let Some(before_sig) = window.most_recent {
            let num_recovered = self
                .parse_transaction_logs_for_missing_events(before_sig)
                .await?;
            recovered_events += num_recovered;
        }

        let transaction_batch_size = self.max_transaction_batch_size();
        loop {
            // Check if we can stop early
            if let Some(stop_after_num_found) = window.stop_after_num_found {
                if recovered_events >= stop_after_num_found {
                    info!("All missing events have been recovered ({} recovered, stopping after finding {}).", recovered_events, stop_after_num_found);
                    return Ok(());
                }
            } else {
                debug!("Found {} events so far", recovered_events);
            }

            let tx_statuses = self
                .rpc_client()
                .get_signatures_for_address_with_config(
                    &target_account,
                    GetConfirmedSignaturesForAddress2Config {
                        before,
                        until: window.least_recent,
                        limit: Some(transaction_batch_size),
                        commitment: Some(CommitmentConfig::finalized()),
                        ..Default::default()
                    },
                )
                .await
                .map_err(|e| Self::Error::from(e))?;

            debug!("Fetched {} signatures.", tx_statuses.len());

            for tx_status in tx_statuses.iter() {
                if tx_status.err.is_some() && skip_failed_transactions {
                    debug!(
                        "Skipping failed transaction {}, which produced error: {:?}",
                        tx_status.signature, tx_status.err
                    );
                    continue;
                }

                let parsed_signature = tx_status.signature.parse().map_err(|_| {
                    ClientError::from(ClientErrorKind::Custom(
                        "RPC server returned an invalid transaction signature".to_string(),
                    ))
                })?;

                let num_recovered = self
                    .parse_transaction_logs_for_missing_events(parsed_signature)
                    .await?;
                before = Some(parsed_signature);
                recovered_events += num_recovered;
            }

            if tx_statuses.len() < transaction_batch_size {
                info!("Searched entire window {:?}", window);
                break;
            }
        }

        // Process the 'until' signature
        if let Some(until_sig) = &until {
            let num_recovered = self
                .parse_transaction_logs_for_missing_events(*until_sig)
                .await
                .map_err(|e| {
                    error!("Error processing signature {}: {}", until_sig, e);
                    e
                })?;
            recovered_events += num_recovered;
        }

        if let Some(stop_after_num_found) = window.stop_after_num_found {
            if recovered_events >= stop_after_num_found {
                info!("All missing events have been recovered ({} recovered, stopping after finding {}).", recovered_events, stop_after_num_found);
            } else {
                warn!("Recovered fewer events in current window than expected ({} recovered, expected to find {}).", recovered_events, stop_after_num_found);
            }
        } else {
            info!(
                "Recovered a total of {} events in window {:?}",
                recovered_events, window
            );
        }

        Ok(())
    }
}
