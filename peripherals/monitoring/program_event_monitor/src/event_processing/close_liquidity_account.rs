use anyhow::{anyhow, Result};
use log::info;
use lollys_lotto::CloseLiquidityAccountEvent;
use program_monitor_db::{utils::type_conversions::i64_to_naive_datetime, Database};

pub async fn process_close_liquidity_account(
    db: &Database,
    data: CloseLiquidityAccountEvent,
    close_event_id: i64,
) -> Result<()> {
    db.update_liquidity_info_close_event_id(
        close_event_id,
        &data.address,
        i64_to_naive_datetime(data.created_at),
    )
    .await
    .map_err(|e| {
        anyhow!(
            "failed to update liquidity account close event ID for address {}: {}",
            data.address,
            e
        )
    })?;

    info!("liquidity account close ID updated: {}", data.address);

    Ok(())
}
