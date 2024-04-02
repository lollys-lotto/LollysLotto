use anyhow::{anyhow, Result};
use lollys_lotto::CreateLiquidityAccountEvent;
use log::info;
use program_monitor_db::{utils::type_conversions::i64_to_naive_datetime, Database};

pub async fn process_create_liquidity_account(
    db: &Database,
    data: CreateLiquidityAccountEvent,
    creation_event_id: i64,
) -> Result<()> {
    db.insert_liquidity_account_info(
        &data.address,
        &data.mint,
        Some(&data.owner),
        Some(&data.pool_registry),
        Some(i64_to_naive_datetime(data.created_at)),
        Some(creation_event_id),
        None,
    )
    .await
    .map_err(|e| {
        anyhow!(
            "failed to insert created liquidity account {}: {}",
            data.address,
            e
        )
    })?;
    info!("liquidity account created: {}", data.address);
    Ok(())
}
