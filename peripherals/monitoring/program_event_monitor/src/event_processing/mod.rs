mod claim_fees;
mod close_liquidity_account;
mod create_liquidity_account;
mod deposit_and_withdraw;
mod internal_swap_event;
mod shared_processing;
mod swap_event;

use crate::{
    address_calculator::AddressCalculator,
    event_processing::{
        claim_fees::process_claim_fee_event,
        close_liquidity_account::process_close_liquidity_account,
        create_liquidity_account::process_create_liquidity_account,
        deposit_and_withdraw::process_deposit_and_withdraw_event,
        internal_swap_event::process_internal_swap_event, swap_event::process_swap_event,
    },
    log_parsing::LoggedTransactionFailure,
};
use anyhow::{anyhow, Result};
use log::info;
use lollys_lotto::{SSLProgramEvent, SSLProgramEventData};
use program_monitor_db::{
    utils::type_conversions::i64_to_naive_datetime, Database, FailedProgramEventRow,
    InstructionType, LollysLottoDatabaseError, ProgramEventRow,
};
use solana_sdk::{clock::Slot, signature::Signature, transaction::TransactionError};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Whenever we encounter an Anchor `Event`, we want to do some processing to it
/// and perform the necessary database insertions.
///
/// This is called whenever we receive transaction logs from an RPC,
/// either during websockets `logs_subscribe`, or during a targeted HTTP `get_transaction`.
///
/// If the function returned false, the event was considered a duplicate.
pub async fn process_event(
    db: Arc<Database>,
    address_calculator: Arc<RwLock<AddressCalculator>>,
    transaction_signature: Signature,
    slot: Slot,
    execution_error: Option<LoggedTransactionFailure>,
    rpc_error_response: Option<TransactionError>,
    raw_event: String,
    event: SSLProgramEvent,
    is_backfilling: bool,
) -> Result<bool> {
    let event_id = event.event_id();
    info!(
        "Event: {} at slot {} transaction {}",
        event_id, slot, transaction_signature
    );
    let ix_type = InstructionType::from(&event);

    // Check if there is an execution error
    if let Some(execution_error) = execution_error {
        // Insert into failed_program_event due to execution error
        let program_event_row = insert_failed_program_event(
            &db,
            event_id,
            event.version(),
            ix_type,
            raw_event,
            &transaction_signature,
            slot,
            event.block_time(),
            Some(execution_error),
            rpc_error_response,
        )
        .await?;

        // Since there's an execution error, do not proceed with indexing specific tables
        match program_event_row {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    } else {
        // No execution error, insert into program_event and process event further
        let program_event_row = insert_program_event(
            &db,
            event_id,
            event.version(),
            ix_type,
            raw_event,
            &transaction_signature,
            slot,
            event.block_time(),
            None,
        )
        .await?;

        if let Some(program_event_row) = program_event_row {
            // Proceed with indexing specific tables since there's no execution error
            index_specific_tables_for_specific_event(
                db,
                address_calculator,
                ix_type,
                event,
                program_event_row,
                is_backfilling,
            )
            .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

async fn index_specific_tables_for_specific_event(
    db: Arc<Database>,
    address_calculator: Arc<RwLock<AddressCalculator>>,
    ix_type: InstructionType,
    event: SSLProgramEvent,
    program_event_row: ProgramEventRow,
    is_backfilling: bool,
) -> Result<()> {
    match event.data {
        SSLProgramEventData::CreateLiquidityAccount(data) => {
            process_create_liquidity_account(&db, data, program_event_row.id).await?;
        }
        SSLProgramEventData::CloseLiquidityAccount(data) => {
            process_close_liquidity_account(&db, data, program_event_row.id).await?;
        }
        SSLProgramEventData::ClaimFees(data) => {
            let mut address_calculator = address_calculator.write().await;
            process_claim_fee_event(&db, &mut address_calculator, program_event_row.id, data)
                .await?;
        }
        SSLProgramEventData::Deposit(data) => {
            let mut address_calculator = address_calculator.write().await;
            process_deposit_and_withdraw_event(
                &db,
                &mut address_calculator,
                program_event_row.id,
                data,
                ix_type,
                is_backfilling,
            )
            .await?;
        }
        SSLProgramEventData::Withdraw(data) => {
            let mut address_calculator = address_calculator.write().await;
            process_deposit_and_withdraw_event(
                &db,
                &mut address_calculator,
                program_event_row.id,
                data,
                ix_type,
                is_backfilling,
            )
            .await?;
        }
        SSLProgramEventData::Swap(data) => {
            let mut address_calculator = address_calculator.write().await;
            process_swap_event(
                &db,
                &mut address_calculator,
                program_event_row.id,
                data,
                is_backfilling,
            )
            .await?;
        }
        SSLProgramEventData::InternalSwap(data) => {
            let mut address_calculator = address_calculator.write().await;
            process_internal_swap_event(
                &db,
                &mut address_calculator,
                program_event_row.id,
                data,
                is_backfilling,
            )
            .await?;
        }
    }
    Ok(())
}

/// Skip duplicates, return `None` for the event row.
async fn insert_program_event(
    db: &Database,
    event_id: i64,
    version: u8,
    ix_type: InstructionType,
    raw_event: String,
    transaction_signature: &Signature,
    slot: u64,
    block_time: i64,
    execution_error: Option<LoggedTransactionFailure>,
) -> Result<Option<ProgramEventRow>> {
    match db
        .insert_program_event(
            event_id,
            version,
            ix_type,
            raw_event,
            transaction_signature,
            slot,
            i64_to_naive_datetime(block_time),
            execution_error.clone().map(|err| err.program),
            execution_error.map(|err| err.error),
        )
        .await
    {
        Ok(row) => Ok(Some(row)),
        Err(LollysLottoDatabaseError::DuplicateKeyValue(constraint, e)) => {
            info!(
                "skipping duplicate event {} {:?} {} due to constraint {}: {}",
                event_id, ix_type, transaction_signature, constraint, e,
            );
            Ok(None)
        }
        Err(e) => Err(anyhow!("error inserting new event: {:?}", e)),
    }
}

/// Skip duplicates, return `None` for the event row.
async fn insert_failed_program_event(
    db: &Database,
    event_id: i64,
    version: u8,
    ix_type: InstructionType,
    raw_event: String,
    transaction_signature: &Signature,
    slot: u64,
    block_time: i64,
    execution_error: Option<LoggedTransactionFailure>,
    rpc_error_response: Option<TransactionError>,
) -> Result<Option<FailedProgramEventRow>> {
    match db
        .insert_failed_program_event(
            event_id,
            version,
            ix_type,
            raw_event,
            transaction_signature,
            slot,
            i64_to_naive_datetime(block_time),
            execution_error.clone().map(|err| err.program),
            execution_error.clone().map(|err| err.error),
            rpc_error_response.map(|err| err.to_string()),
        )
        .await
    {
        Ok(row) => Ok(Some(row)),
        Err(LollysLottoDatabaseError::DuplicateKeyValue(constraint, e)) => {
            info!(
                "skipping duplicate failed event {} {:?} {} due to constraint {}: {}",
                event_id, ix_type, transaction_signature, constraint, e,
            );
            Ok(None)
        }
        Err(e) => Err(anyhow!("error inserting new failed event: {:?}", e)),
    }
}
