use crate::{
    address_calculator::AddressCalculator,
    event_processing::shared_processing::{self},
    prom_metrics::metrics::set_pool_vault_balance_metrics,
};
use log::{info, warn};
use lollys_lotto::InternalSwapEvent;
use program_monitor_db::{error::ForeignKey, Database, LollysLottoDatabaseError};

pub async fn process_internal_swap_event(
    db: &Database,
    address_calculator: &mut AddressCalculator,
    event_id: i64,
    event: InternalSwapEvent,
    is_backfilling: bool,
) -> anyhow::Result<()> {
    let mint_a = address_calculator
        .mint_at_ssl_entry(event.mint_a as usize)
        .await?;
    let mint_b = address_calculator
        .mint_at_ssl_entry(event.mint_b as usize)
        .await?;
    let pair = address_calculator.pair(&mint_a, &mint_b);

    // mint_a ssl_a, main vault
    shared_processing::process_pool_vault_activity(
        db,
        address_calculator,
        event_id,
        &mint_a,
        None,
        event.ssl_a_main_vault_balance_before,
        event.ssl_a_main_vault_balance_after,
    )
    .await?;

    // mint_a mint ssl_a, secondary vault
    shared_processing::process_pool_vault_activity(
        db,
        address_calculator,
        event_id,
        &mint_a,
        Some(&mint_b),
        event.ssl_a_secondary_vault_balance_before,
        event.ssl_a_secondary_vault_balance_after,
    )
    .await?;

    // mint_b mint ssl_b, main vault
    shared_processing::process_pool_vault_activity(
        db,
        address_calculator,
        event_id,
        &mint_b,
        None,
        event.ssl_b_main_vault_balance_before,
        event.ssl_b_main_vault_balance_after,
    )
    .await?;

    // mint_b mint ssl_b, secondary vault
    shared_processing::process_pool_vault_activity(
        db,
        address_calculator,
        event_id,
        &mint_b,
        Some(&mint_a),
        event.ssl_b_secondary_vault_balance_before,
        event.ssl_b_secondary_vault_balance_after,
    )
    .await?;

    if !is_backfilling {
        set_pool_vault_balance_metrics(
            db,
            &mint_a,
            &[
                &mint_a.to_string(),
                "",
                &address_calculator.pool_vault(&mint_a, None).to_string(),
            ],
            event.ssl_a_main_vault_balance_after,
        )
        .await?;

        set_pool_vault_balance_metrics(
            db,
            &mint_b,
            &[
                &mint_a.to_string(),
                &mint_b.to_string(),
                &address_calculator
                    .pool_vault(&mint_a, Some(&mint_b))
                    .to_string(),
            ],
            event.ssl_a_secondary_vault_balance_after,
        )
        .await?;

        set_pool_vault_balance_metrics(
            db,
            &mint_b,
            &[
                &mint_b.to_string(),
                "",
                &address_calculator.pool_vault(&mint_b, None).to_string(),
            ],
            event.ssl_b_main_vault_balance_after,
        )
        .await?;

        set_pool_vault_balance_metrics(
            db,
            &mint_a,
            &[
                &mint_b.to_string(),
                &mint_a.to_string(),
                &address_calculator
                    .pool_vault(&mint_b, Some(&mint_a))
                    .to_string(),
            ],
            event.ssl_b_secondary_vault_balance_after,
        )
        .await?;
    }

    match db
        .insert_internal_swap_activity(
            event_id,
            &pair,
            event.token_a_price,
            event.token_b_price,
            event.ssl_a_internally_swapped_volume,
            event.ssl_b_internally_swapped_volume,
        )
        .await
    {
        Err(LollysLottoDatabaseError::MissingForeignKey(ForeignKey::Pair, _)) => {
            warn!("Missing pair foreign key");
            db.insert_pair_info(&address_calculator.pool_registry, &mint_a, &mint_b)
                .await?;
            db.insert_internal_swap_activity(
                event_id,
                &pair,
                event.token_a_price,
                event.token_b_price,
                event.ssl_a_internally_swapped_volume,
                event.ssl_b_internally_swapped_volume,
            )
            .await?;
        }
        Err(LollysLottoDatabaseError::DuplicateKeyValue(constraint, _)) => {
            warn!(
                "violated constraint {}, skipping duplicate internal swap tied to event {}",
                constraint, event_id
            );
        }
        Err(e) => {
            return Err(e.into());
        }
        Ok(_) => {}
    }
    info!("new internal swap event: {}", event_id);
    Ok(())
}
