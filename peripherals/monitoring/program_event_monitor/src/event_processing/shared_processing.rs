use crate::address_calculator::AddressCalculator;
use lollys_lotto::Pair;
use log::{info, warn};
use program_monitor_db::{
    error::ForeignKey, utils::type_conversions::option_pubkey::Pubkey, Database, SSLv2DatabaseError,
};

pub async fn process_fee_vault_activity(
    db: &Database,
    address_calculator: &AddressCalculator,
    event_id: i64,
    mint_out: &Pubkey,
    balance_before: u64,
    balance_after: u64,
) -> anyhow::Result<()> {
    let fee_vault = address_calculator.fee_vault(mint_out);
    match db
        .insert_fee_vault_activity(event_id, &fee_vault, balance_before, balance_after)
        .await
    {
        Err(SSLv2DatabaseError::MissingForeignKey(ForeignKey::FeeVault, _)) => {
            info!("Missing fee vault foreign key");
            db.insert_fee_vault_info(&fee_vault, &address_calculator.pool_registry, &mint_out)
                .await?;
            db.insert_fee_vault_activity(event_id, &fee_vault, balance_before, balance_after)
                .await?;
        }
        Err(SSLv2DatabaseError::DuplicateKeyValue(constraint, _)) => {
            warn!(
                "violated constraint {}, skipping duplicate fee vault activity tied to event {}",
                constraint, event_id
            );
        }
        Err(e) => {
            return Err(e.into());
        }
        Ok(_) => {}
    }
    Ok(())
}

pub async fn process_pool_vault_activity(
    db: &Database,
    address_calculator: &AddressCalculator,
    event_id: i64,
    mint: &Pubkey,
    secondary_mint: Option<&Pubkey>,
    balance_before: u64,
    balance_after: u64,
) -> anyhow::Result<()> {
    let pool_vault = address_calculator.pool_vault(mint, secondary_mint);
    match db
        .insert_pool_vault_activity(event_id, &pool_vault, balance_before, balance_after)
        .await
    {
        Err(SSLv2DatabaseError::MissingForeignKey(ForeignKey::PoolVault, _)) => {
            info!("Missing pool vault foreign key");
            db.insert_pool_vault_info(
                &pool_vault,
                &address_calculator.pool_registry,
                &mint,
                secondary_mint,
            )
            .await?;
            db.insert_pool_vault_activity(event_id, &pool_vault, balance_before, balance_after)
                .await?;
        }
        Err(SSLv2DatabaseError::DuplicateKeyValue(constraint, _)) => {
            warn!(
                "violated constraint {}, skipping duplicate pool vault activity tied to event {}",
                constraint, event_id
            );
        }
        Err(e) => {
            return Err(e.into());
        }
        Ok(_) => {}
    }
    Ok(())
}

pub async fn process_liquidity_account_activity(
    db: &Database,
    event_id: i64,
    liquidity_account: &Pubkey,
    mint: &Pubkey,
    amount_deposited: u64,
    last_observed_tap: u64,
    total_earned: u64,
) -> anyhow::Result<()> {
    match db
        .insert_liquidity_account_activity(
            event_id,
            &liquidity_account,
            amount_deposited,
            last_observed_tap,
            total_earned,
        )
        .await
    {
        Err(SSLv2DatabaseError::MissingForeignKey(ForeignKey::LiquidityAccount, _)) => {
            info!("Missing liquidity account foreign key");
            db.insert_liquidity_account_info(
                &liquidity_account,
                mint,
                None,
                None,
                None,
                None,
                None,
            )
            .await?;
            db.insert_liquidity_account_activity(
                event_id,
                &liquidity_account,
                amount_deposited,
                last_observed_tap,
                total_earned,
            )
            .await?;
        }
        Err(SSLv2DatabaseError::DuplicateKeyValue(constraint, _)) => {
            warn!(
                "violated constraint {}, skipping duplicate liquidity activity tied to event {}",
                constraint, event_id
            );
        }
        Err(e) => {
            return Err(e.into());
        }
        Ok(_) => {}
    }
    Ok(())
}

pub async fn process_lp_deposit_activity(
    db: &Database,
    event_id: i64,
    liquidity_account: &Pubkey,
    address_calculator: &AddressCalculator,
    mint: &Pubkey,
    total_liquidity_deposited: u64,
) -> anyhow::Result<()> {
    match db
        .insert_lp_deposit_activity(
            event_id,
            &address_calculator.pool_registry,
            mint,
            total_liquidity_deposited,
        )
        .await
    {
        Err(SSLv2DatabaseError::MissingForeignKey(ForeignKey::LiquidityAccount, _)) => {
            info!("Missing liquidity account foreign key");
            db.insert_liquidity_account_info(
                &liquidity_account,
                mint,
                None,
                None,
                None,
                None,
                None,
            )
            .await?;
            db.insert_lp_deposit_activity(
                event_id,
                &address_calculator.pool_registry,
                mint,
                total_liquidity_deposited,
            )
            .await?;
        }
        Err(SSLv2DatabaseError::DuplicateKeyValue(constraint, _)) => {
            warn!(
                "violated constraint {}, skipping duplicate lp deposit activity tied to event {}",
                constraint, event_id
            );
        }
        Err(e) => {
            return Err(e.into());
        }
        Ok(_) => {}
    }
    Ok(())
}

pub async fn process_pair_activity(
    db: &Database,
    event_id: i64,
    address_calculator: &AddressCalculator,
    mint_in: Pubkey,
    mint_out: Pubkey,
    fee_vault_mint: Pubkey,
    total_fees_generated: u128,
    total_historical_volume: u128,
) -> anyhow::Result<()> {
    let (mint_a, _) = Pair::normalize_mint_order(mint_in, mint_out);
    let fees_generated_is_mint_a = fee_vault_mint == mint_a;
    match db
        .insert_pair_activity(
            event_id,
            &address_calculator.pair(&mint_in, &mint_out),
            fees_generated_is_mint_a,
            total_fees_generated,
            total_historical_volume,
        )
        .await
    {
        Err(SSLv2DatabaseError::MissingForeignKey(ForeignKey::Pair, _)) => {
            info!("Missing pair foreign key");
            db.insert_pair_info(&address_calculator.pool_registry, &mint_in, &mint_out)
                .await?;
            db.insert_pair_activity(
                event_id,
                &address_calculator.pair(&mint_in, &mint_out),
                fees_generated_is_mint_a,
                total_fees_generated,
                total_historical_volume,
            )
            .await?;
        }
        Err(SSLv2DatabaseError::DuplicateKeyValue(constraint, _)) => {
            warn!(
                "violated constraint {}, skipping duplicate pair activity tied to event {}",
                constraint, event_id
            );
        }
        Err(e) => {
            return Err(e.into());
        }
        Ok(_) => {}
    }
    Ok(())
}
