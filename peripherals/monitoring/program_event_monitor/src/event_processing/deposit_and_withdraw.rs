use crate::{
    address_calculator::AddressCalculator,
    event_processing::shared_processing::{self},
    prom_metrics::metrics::{set_market_making_pnl_metrics, set_pool_vault_balance_metrics},
};
use log::info;
use lollys_lotto::DepositOrWithdraw;
use program_monitor_db::{Database, InstructionType};

pub async fn process_deposit_and_withdraw_event(
    db: &Database,
    address_calculator: &mut AddressCalculator,
    event_id: i64,
    event: DepositOrWithdraw,
    ix_type: InstructionType,
    is_backfilling: bool,
) -> anyhow::Result<()> {
    let (main_vault_balance_before, main_vault_balance_after) = event.main_vault_balance;
    let (_, total_liquidity_deposits_after) = event.total_liquidity_deposits;
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

    shared_processing::process_lp_deposit_activity(
        db,
        event_id,
        &event.address,
        address_calculator,
        &event.mint,
        total_liquidity_deposits_after,
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

    shared_processing::process_pool_vault_activity(
        db,
        address_calculator,
        event_id,
        &event.mint,
        None,
        main_vault_balance_before,
        main_vault_balance_after,
    )
    .await?;

    // Set SSL metrics for prometheus for non-backfill events
    if !is_backfilling {
        set_pool_vault_balance_metrics(
            db,
            &event.mint,
            &[
                &event.mint.to_string(),
                "",
                &address_calculator.pool_vault(&event.mint, None).to_string(),
            ],
            main_vault_balance_after,
        )
        .await?;

        set_market_making_pnl_metrics(
            db,
            address_calculator,
            &event.mint,
            &[&event.mint.to_string()],
        )
        .await?;
    }

    let variant = match ix_type {
        InstructionType::Deposit => "deposit",
        InstructionType::Withdraw => "withdraw",
        _ => unreachable!(),
    };
    info!(
        "new {variant} event: {event_id} for liquidity account: {}",
        event.address
    );
    Ok(())
}
