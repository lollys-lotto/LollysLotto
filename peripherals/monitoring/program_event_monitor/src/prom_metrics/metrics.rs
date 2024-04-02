use futures::{future::ready, FutureExt};
use lollys_lotto::{sdk::constants::ConstMint, Pair};
use hyper::{
    server::Server,
    service::{make_service_fn, service_fn},
    Body, Response,
};
use once_cell::sync::Lazy;
use program_monitor_db::Database;
use prometheus::{
    opts, register_gauge_vec_with_registry, Encoder, GaugeVec, Registry, TextEncoder,
};
use solana_sdk::pubkey::Pubkey;
use std::{convert::Infallible, net::SocketAddr};

use crate::address_calculator::AddressCalculator;

/// Define the custom registry and all metrics as Lazy static variables
pub static SSL_METRICS_REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

/// Helper function to register metrics with the custom registry
fn register_metric<'a>(name: &'a str, description: &'a str, labels: &[&str]) -> GaugeVec {
    register_gauge_vec_with_registry!(
        opts!(name, description),
        labels,
        SSL_METRICS_REGISTRY.clone()
    )
    .expect("metric registration failed")
}

pub static POOL_VAULT_BALANCE_NATIVE: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "pool_vault_balance_native",
        "The current SSL pool vault balance in native mint",
        &["main_mint", "secondary_mint", "pool_vault_address"],
    )
});

pub static POOL_VAULT_BALANCE_USD: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "pool_vault_balance_usd",
        "The current SSL pool vault balance in $USD",
        &["main_mint", "secondary_mint", "pool_vault_address"],
    )
});

pub static PAIR_VOLUME: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "pair_volume",
        "The total historical volume of a Pair",
        &["mint_a", "mint_b", "pair_address"],
    )
});

pub static PAIR_GENERATED_FEE: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "pair_generated_fee",
        "The total generated fees for a Pair",
        &["mint_a", "mint_b", "pair_address"],
    )
});

pub static ORACLE_PRICE: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "oracle_price",
        "The current oracle price for a mint",
        &["mint", "oracle_address"],
    )
});

pub static FEE_VAULT_BALANCE: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "fee_vault_balance",
        "The current SSL fee vault balance",
        &["mint", "fee_vault_address"],
    )
});

pub static POOL_MARKET_MAKING_PNL_USD: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "pool_market_making_pnl_usd",
        "The current market making pnl of a pool in $USD",
        &["mint"],
    )
});

pub static POOL_MARKET_MAKING_PNL_USD_PCT: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "pool_market_making_pnl_usd_pct",
        "The current market making pnl percentage of a pool in terms of $USD",
        &["mint"],
    )
});

pub static POOL_TVL_USD: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "pool_tvl_usd",
        "The current tvl of a pool in $USD",
        &["mint"],
    )
});

pub static MINT_LIQUIDITY_DEPOSITS_NATIVE: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "mint_liquidity_deposits_native",
        "The current native liquidity deposits of a mint",
        &["mint"],
    )
});

pub static MINT_LIQUIDITY_DEPOSITS_USD: Lazy<GaugeVec> = Lazy::new(|| {
    register_metric(
        "mint_liquidity_deposits_usd",
        "The current liquidity deposits of a mint in $USD",
        &["mint"],
    )
});

pub async fn serve(custom_registry: Registry, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let make_service = make_service_fn(move |_| {
        let registry = custom_registry.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |_req| {
                let registry = registry.clone();
                async move {
                    let mut buffer = vec![];
                    let encoder = TextEncoder::new();
                    // Use the custom registry to gather metrics
                    let metric_families = registry.gather();
                    encoder.encode(&metric_families, &mut buffer).unwrap();

                    Ok::<_, Infallible>(Response::new(Body::from(buffer)))
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    server
        .then(|r| {
            if let Err(e) = r {
                eprintln!("[Server] Exit with error: {}", e);
            }
            ready(())
        })
        .await;
}

pub async fn calculate_latest_mint_usd_value(
    db: &Database,
    mint: &Pubkey,
    native_amount: u64,
) -> anyhow::Result<Option<(f64, f64)>> {
    let price_entry = db.fetch_latest_entry_by_mint(mint).await?;

    if let Some(oracle_price) = price_entry {
        let mint_info = <&'static ConstMint as TryFrom<&Pubkey>>::try_from(mint).map_err(|e| {
            anyhow::Error::msg(format!("Failed to find ConstMint for pubkey: {:?}", e))
        })?;

        let raw_value = (native_amount as f64) / (10f64.powi(mint_info.decimals as i32));
        let usd_value = raw_value
            * ((oracle_price.price_num as f64) / (10f64.powi(oracle_price.price_scale as i32)));
        Ok(Some((usd_value, raw_value)))
    } else {
        Ok(None)
    }
}

pub async fn get_pool_vault_balance_native_usd(
    db: &Database,
    mint: &Pubkey,
    pool_vault_address: &Pubkey,
) -> anyhow::Result<Option<(f64, f64)>> {
    if let Ok(activity) = db
        .fetch_latest_pool_vault_activity_by_mint(&pool_vault_address)
        .await
    {
        Ok(calculate_latest_mint_usd_value(db, &mint, activity.balance_after).await?)
    } else {
        Ok(None)
    }
}

pub async fn get_pool_total_liquidity_deposits_native_usd(
    db: &Database,
    mint: &Pubkey,
) -> anyhow::Result<Option<(f64, f64)>> {
    if let Ok(activity) = db
        .fetch_latest_lp_deposit_vault_activity_by_mint(&mint)
        .await
    {
        Ok(calculate_latest_mint_usd_value(db, &mint, activity.total_liquidity_deposited).await?)
    } else {
        Ok(None)
    }
}

pub async fn set_pool_vault_balance_metrics(
    db: &Database,
    mint: &Pubkey,
    label_values: &[&str],
    pool_vault_balance_native: u64,
) -> anyhow::Result<()> {
    if let Some((usd_value, native_value)) =
        calculate_latest_mint_usd_value(db, mint, pool_vault_balance_native).await?
    {
        POOL_VAULT_BALANCE_NATIVE
            .with_label_values(label_values)
            .set(native_value);
        POOL_VAULT_BALANCE_USD
            .with_label_values(label_values)
            .set(usd_value);
    }
    Ok(())
}

pub async fn set_pair_metrics(
    db: &Database,
    mint_out: &Pubkey,
    label_values: &[&str],
    total_historical_volume: u128,
    total_fees_generated: u128,
) -> anyhow::Result<()> {
    let pair_total_historical_volume_usd =
        (total_historical_volume as f64) / (10f64.powi(Pair::USD_VOLUME_DECIMALS as i32));

    PAIR_VOLUME
        .with_label_values(label_values)
        .set(pair_total_historical_volume_usd);

    if let Some((usd_value, _)) =
        calculate_latest_mint_usd_value(db, &mint_out, total_fees_generated.try_into().unwrap())
            .await?
    {
        PAIR_GENERATED_FEE
            .with_label_values(label_values)
            .set(usd_value);
    }

    Ok(())
}

pub async fn set_market_making_pnl_metrics(
    db: &Database,
    address_calculator: &mut AddressCalculator,
    mint: &Pubkey,
    label_values: &[&str],
) -> anyhow::Result<()> {
    let pool_registry = address_calculator.pool_registry_data;
    let main_pool_vault = address_calculator.main_vault(mint);
    let mut vaults_hashmap =
        pool_registry.secondary_vault_addresses(address_calculator.pool_registry, mint.clone());
    vaults_hashmap.insert(*mint, main_pool_vault);

    let mut pool_tvl_usd = 0f64;

    for (mint_key, pool_vault_address) in vaults_hashmap {
        if let Some((usd_value, _)) =
            get_pool_vault_balance_native_usd(db, &mint_key, &pool_vault_address).await?
        {
            pool_tvl_usd += usd_value;
        }
    }

    if let Some((total_liquidity_deposited_usd, total_liquidity_deposited_native)) =
        get_pool_total_liquidity_deposits_native_usd(db, mint).await?
    {
        MINT_LIQUIDITY_DEPOSITS_NATIVE
            .with_label_values(label_values)
            .set(total_liquidity_deposited_native);
        MINT_LIQUIDITY_DEPOSITS_USD
            .with_label_values(label_values)
            .set(total_liquidity_deposited_usd);

        // Calculate the market making profit or loss in USD
        let market_making_pnl_usd = pool_tvl_usd - total_liquidity_deposited_usd;
        POOL_MARKET_MAKING_PNL_USD
            .with_label_values(label_values)
            .set(market_making_pnl_usd);

        // Calculate the percentage of market making profit or loss
        let market_making_pnl_pct = if total_liquidity_deposited_usd != 0.0 {
            (market_making_pnl_usd / total_liquidity_deposited_usd) * 100.0
        } else {
            0.0
        };
        POOL_MARKET_MAKING_PNL_USD_PCT
            .with_label_values(label_values)
            .set(market_making_pnl_pct);
    }

    POOL_TVL_USD
        .with_label_values(label_values)
        .set(pool_tvl_usd);

    Ok(())
}
