use crate::{address_calculator::AddressCalculator, event_processing::shared_processing};
use log::info;
use lollys_lotto::ClaimFeeEvent;
use program_monitor_db::Database;

pub async fn process_claim_fee_event(
    db: &Database,
    address_calculator: &mut AddressCalculator,
    event_id: i64,
    event: ClaimFeeEvent,
) -> anyhow::Result<()> {
    let (fee_vault_balance_before, fee_vault_balance_after) = event.fee_vault_balance;
    shared_processing::process_liquidity_account_activity(
        db,
        event_id,
        &event.address,
        &event.mint,
        event.amount_deposited,
        event.last_observed_tap,
        event.total_earned,
    )
    .await?;
    shared_processing::process_fee_vault_activity(
        db,
        address_calculator,
        event_id,
        &event.mint,
        fee_vault_balance_before,
        fee_vault_balance_after,
    )
    .await?;

    info!("new claim fee event: {}", event_id);
    Ok(())
}
