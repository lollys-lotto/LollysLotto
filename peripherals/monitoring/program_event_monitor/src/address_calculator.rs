use anchor_lang::AccountDeserialize;
use anyhow::{anyhow, Result};
use lollys_lotto::{Pair, PoolRegistry, SSLPool};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use std::sync::Arc;
use tokio::sync::RwLock;

/// For PDA derivation, and for resolving other address information,
/// e.g., which mint resides on a particular `SSLPool` entry.
#[derive(Clone)]
pub struct AddressCalculator {
    pub pool_registry: Pubkey,
    pub pool_registry_data: PoolRegistry,
    /// Non-blocking client
    pub client: Arc<RwLock<RpcClient>>,
}

impl AddressCalculator {
    pub async fn new(client: RpcClient, pool_registry: Pubkey) -> Result<Self> {
        let pool_registry_data = get_pool_registry(&pool_registry, &client).await?;
        Ok(Self {
            pool_registry,
            pool_registry_data,
            client: Arc::new(RwLock::new(client)),
        })
    }

    /// Update `self.pool_registry_data` with the current on-chain data.
    pub async fn refresh_pool_registry_data(&mut self) -> Result<()> {
        let client = self.client.read().await;
        self.pool_registry_data = get_pool_registry(&self.pool_registry, &client).await?;
        Ok(())
    }

    /// Given a lookup index on some `pool_registry.entries`, return its mint.
    /// If not found, try refreshing once, as we may have added a new SSL pool.
    pub async fn mint_at_ssl_entry(&mut self, index: usize) -> Result<Pubkey> {
        let mint = mint_at_entry(&self.pool_registry_data, index);
        if let Some(mint) = mint {
            Ok(mint)
        } else {
            self.refresh_pool_registry_data().await?;
            mint_at_entry(&self.pool_registry_data, index).ok_or(anyhow!(
                "No mint in pool registry {} at index {}",
                &self.pool_registry,
                index,
            ))
        }
    }

    pub fn pool_vault(&self, main_mint: &Pubkey, secondary_mint: Option<&Pubkey>) -> Pubkey {
        if let Some(secondary_mint) = secondary_mint {
            self.secondary_vault(main_mint, secondary_mint)
        } else {
            self.main_vault(main_mint)
        }
    }

    pub fn main_vault(&self, mint: &Pubkey) -> Pubkey {
        SSLPool::vault_address(self.pool_registry, *mint)
    }

    pub fn secondary_vault(&self, main_mint: &Pubkey, secondary_mint: &Pubkey) -> Pubkey {
        SSLPool::secondary_token_vault_address(self.pool_registry, *main_mint, *secondary_mint)
    }

    pub fn fee_vault(&self, mint: &Pubkey) -> Pubkey {
        get_associated_token_address(&self.pool_registry, mint)
    }

    pub fn pair(&self, mint_a: &Pubkey, mint_b: &Pubkey) -> Pubkey {
        Pair::address(self.pool_registry, *mint_a, *mint_b)
    }
}

/// Fetch a `PoolRegistry` using a non-blocking `RpcClient`.
async fn get_pool_registry(pubkey: &Pubkey, client: &RpcClient) -> Result<PoolRegistry> {
    let data = client
        .get_account_data(pubkey)
        .await
        .map_err(|e| anyhow!("Failed to fetch pool registry {}: {e}", pubkey))?;
    PoolRegistry::try_deserialize(&mut data.as_slice())
        .map_err(|e| anyhow!("Failed to deserialize pool registry {}, {e}", pubkey))
}

/// Given a lookup index on some `pool_registry.entries`, return its mint.
pub fn mint_at_entry(pool_registry: &PoolRegistry, index: usize) -> Option<Pubkey> {
    match pool_registry.entries.get(index) {
        Some(pool) => {
            if *pool == SSLPool::default() {
                return None;
            }
            Some(pool.mint)
        }
        None => None,
    }
}
