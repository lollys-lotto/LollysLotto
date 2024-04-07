use crate::{
    address_calculator::AddressCalculator,
    event_processing::shared_processing,
    prom_metrics::metrics::{
        set_market_making_pnl_metrics, set_pair_metrics, set_pool_vault_balance_metrics,
    },
};
use log::{info, warn};
use lollys_lotto::SwapEvent;
use program_monitor_db::{error::ForeignKey, Database, LollysLottoDatabaseError};

pub async fn process_swap_event(
    db: &Database,
    address_calculator: &mut AddressCalculator,
    sql_primary_key: i64,
    event: SwapEvent,
    is_backfilling: bool,
) -> anyhow::Result<()> {
    let mint_in = address_calculator
        .mint_at_ssl_entry(event.mint_in as usize)
        .await?;
    let mint_out = address_calculator
        .mint_at_ssl_entry(event.mint_out as usize)
        .await?;
    let pair = address_calculator.pair(&mint_in, &mint_out);

    let fee_vault_mint = address_calculator
        .mint_at_ssl_entry(event.fee_vault_mint as usize)
        .await?;

    shared_processing::process_fee_vault_activity(
        db,
        address_calculator,
        sql_primary_key,
        &mint_out,
        event.fee_vault_balance_before,
        event.fee_vault_balance_after,
    )
    .await?;

    // Input mint, main vault
    shared_processing::process_pool_vault_activity(
        db,
        address_calculator,
        sql_primary_key,
        &mint_in,
        None,
        event.input_ssl_main_vault_balance_before,
        event.input_ssl_main_vault_balance_after,
    )
    .await?;

    // Input mint, secondary vault
    shared_processing::process_pool_vault_activity(
        db,
        address_calculator,
        sql_primary_key,
        &mint_in,
        Some(&mint_out),
        event.input_ssl_secondary_vault_balance_before,
        event.input_ssl_secondary_vault_balance_after,
    )
    .await?;

    // Output mint, main vault
    shared_processing::process_pool_vault_activity(
        db,
        address_calculator,
        sql_primary_key,
        &mint_out,
        None,
        event.output_ssl_main_vault_balance_before,
        event.output_ssl_main_vault_balance_after,
    )
    .await?;

    // Output mint, secondary vault
    shared_processing::process_pool_vault_activity(
        db,
        address_calculator,
        sql_primary_key,
        &mint_out,
        Some(&mint_in),
        event.output_ssl_secondary_vault_balance_before,
        event.output_ssl_secondary_vault_balance_after,
    )
    .await?;

    // insert pair activity
    shared_processing::process_pair_activity(
        db,
        sql_primary_key,
        address_calculator,
        mint_in,
        mint_out,
        fee_vault_mint,
        event.total_fees_generated,
        event.total_historical_volume,
    )
    .await?;

    // Set SSL metrics for prometheus for non-backfill events
    if !is_backfilling {
        // Input mint, main vault
        set_pool_vault_balance_metrics(
            db,
            &mint_in,
            &[
                &mint_in.to_string(),
                "",
                &address_calculator.pool_vault(&mint_in, None).to_string(),
            ],
            event.input_ssl_main_vault_balance_after,
        )
        .await?;

        // Input mint, secondary vault
        set_pool_vault_balance_metrics(
            db,
            &mint_out,
            &[
                &mint_in.to_string(),
                &mint_out.to_string(),
                &address_calculator
                    .pool_vault(&mint_in, Some(&mint_out))
                    .to_string(),
            ],
            event.input_ssl_secondary_vault_balance_after,
        )
        .await?;

        // Output mint, main vault
        set_pool_vault_balance_metrics(
            db,
            &mint_out,
            &[
                &mint_out.to_string(),
                "",
                &address_calculator.pool_vault(&mint_out, None).to_string(),
            ],
            event.output_ssl_main_vault_balance_after,
        )
        .await?;

        // Output mint, secondary vault
        set_pool_vault_balance_metrics(
            db,
            &mint_in,
            &[
                &mint_out.to_string(),
                &mint_in.to_string(),
                &address_calculator
                    .pool_vault(&mint_out, Some(&mint_in))
                    .to_string(),
            ],
            event.output_ssl_secondary_vault_balance_after,
        )
        .await?;

        set_pair_metrics(
            db,
            &mint_out,
            &[
                &mint_in.to_string(),
                &mint_out.to_string(),
                &address_calculator.pair(&mint_in, &mint_out).to_string(),
            ],
            event.total_historical_volume,
            event.total_fees_generated,
        )
        .await?;

        set_market_making_pnl_metrics(db, address_calculator, &mint_in, &[&mint_in.to_string()])
            .await?;

        set_market_making_pnl_metrics(db, address_calculator, &mint_out, &[&mint_out.to_string()])
            .await?;
    }

    match db
        .insert_swap_activity(
            sql_primary_key,
            &pair,
            &event.user,
            &mint_in,
            &mint_out,
            event.swap_quote.amount_in,
            event.swap_quote.preswap_pool_token_ratio,
            event.swap_quote.initially_below_max_token_ratio,
            event.swap_quote.can_take_preferred_route,
            event.swap_quote.violates_pool_token_ratio_constraints,
            event.swap_quote.optimal_price,
            event.swap_quote.panic_price,
            event.swap_quote.amount_out_optimal,
            event.swap_quote.fees_out_optimal,
            event.swap_quote.amount_out_panic,
            event.swap_quote.fees_out_panic,
            event.total_accumulated_lp_reward,
        )
        .await
    {
        Err(LollysLottoDatabaseError::MissingForeignKey(ForeignKey::Pair, _)) => {
            warn!("Missing pair foreign key");
            db.insert_pair_info(&address_calculator.pool_registry, &mint_in, &mint_out)
                .await?;
            db.insert_swap_activity(
                sql_primary_key,
                &pair,
                &event.user,
                &mint_in,
                &mint_out,
                event.swap_quote.amount_in,
                event.swap_quote.preswap_pool_token_ratio,
                event.swap_quote.initially_below_max_token_ratio,
                event.swap_quote.can_take_preferred_route,
                event.swap_quote.violates_pool_token_ratio_constraints,
                event.swap_quote.optimal_price,
                event.swap_quote.panic_price,
                event.swap_quote.amount_out_optimal,
                event.swap_quote.fees_out_optimal,
                event.swap_quote.amount_out_panic,
                event.swap_quote.fees_out_panic,
                event.total_accumulated_lp_reward,
            )
            .await?;
        }
        Err(LollysLottoDatabaseError::DuplicateKeyValue(constraint, _)) => {
            warn!(
                "violated constraint {}, skipping duplicate swap tied to event {}",
                constraint, sql_primary_key
            );
        }
        Err(e) => {
            return Err(e.into());
        }
        Ok(_) => {}
    }

    info!("new swap event: {}", sql_primary_key);
    Ok(())
}
