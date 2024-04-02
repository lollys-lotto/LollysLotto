use anyhow::{anyhow, Result};
use lollys_lotto::{sdk::constants::ConstMint, PoolRegistry, SSLPool};
use program_monitor_db::utils::type_conversions::option_pubkey::Pubkey;
use std::collections::{HashMap, HashSet};

pub fn parse_pairs(pairs: Vec<String>, pool_registry_address: &Pubkey) -> Result<Vec<Pubkey>> {
    pairs
        .iter()
        .map(|p| {
            ConstMint::key_of_pair(pool_registry_address, p)
                .ok_or(anyhow!("Unknown pair name: {}", p))
                .map(|pair_and_mints| pair_and_mints.0)
        })
        .collect()
}

pub fn parse_mints(mints: &[String]) -> Result<Vec<Pubkey>> {
    mints
        .iter()
        .map(|m| ConstMint::key_of(m).map_err(Into::into))
        .collect()
}

pub fn filter_pairs(
    all_pairs: HashMap<Pubkey, (Pubkey, Pubkey)>,
    pairs: Vec<Pubkey>,
    mints: Vec<Pubkey>,
) -> Result<Vec<(Pubkey, String)>> {
    if pairs.is_empty() && mints.is_empty() {
        return Ok(all_pairs
            .into_iter()
            .filter(|_| true)
            .map(|(p, (m1, m2))| (p, ConstMint::name_of_pair(&m1, &m2).unwrap()))
            .collect());
    }
    Ok(all_pairs
        .into_iter()
        .filter(|(pair, (mint_a, mint_b))| {
            pairs.contains(&pair) || mints.contains(&mint_a) || mints.contains(&mint_b)
        })
        .map(|(p, (m1, m2))| (p, ConstMint::name_of_pair(&m1, &m2).unwrap()))
        .collect())
}

pub fn parse_secondary_vault(name: &str) -> Result<(Pubkey, Pubkey)> {
    let (main_mint, secondary_mint) = name.split_once("-").ok_or(anyhow!(
        "unknown secondary mint: {name}, must be formatted as MAIN-SECONDARY (e.g. SOL-USDC)"
    ))?;
    let main_mint = ConstMint::key_of(main_mint)?;
    let secondary_mint = ConstMint::key_of(secondary_mint)?;
    Ok((main_mint, secondary_mint))
}

pub fn parse_secondary_vaults(vaults: &[String]) -> Result<Vec<(Pubkey, Pubkey)>> {
    vaults
        .iter()
        .map(|v| parse_secondary_vault(v.as_str()))
        .collect::<Result<Vec<_>>>()
}

pub fn resolve_vaults(
    ssl_pools: &[Pubkey],
    main_vaults: &[Pubkey],
    secondary_vaults: &[(Pubkey, Pubkey)],
    pool_registry: &Pubkey,
    pool_registry_data: PoolRegistry,
) -> Vec<(Pubkey, String)> {
    let mut vaults = HashSet::new();
    // All vaults for a specific pool
    for main_mint in ssl_pools {
        vaults.insert((
            SSLPool::vault_address(*pool_registry, *main_mint),
            format!("{} main vault", ConstMint::name_of(main_mint).unwrap()),
        ));
        vaults.extend(
            pool_registry_data.secondary_vault_addresses_with_names(*pool_registry, *main_mint),
        );
    }
    // All main vaults for specified mints
    for main_mint in main_vaults {
        vaults.insert((
            SSLPool::vault_address(*pool_registry, *main_mint),
            format!("{} main vault", ConstMint::name_of(main_mint).unwrap()),
        ));
    }
    // All specified secondary vaults
    for (main_mint, secondary_mint) in secondary_vaults {
        vaults.insert((
            SSLPool::secondary_token_vault_address(*pool_registry, *main_mint, *secondary_mint),
            format!(
                "{} pool {} vault",
                ConstMint::name_of(main_mint).unwrap(),
                ConstMint::name_of(secondary_mint).unwrap(),
            ),
        ));
    }
    vaults.into_iter().collect()
}
