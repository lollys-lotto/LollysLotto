use crate::{
    error::Result,
    utils::type_conversions::{option_pubkey, pubkey},
    Database, TypedQuery,
};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sqlx::query_as;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct PoolVaultInfoRaw {
    pub address: String,
    pub pool_registry: String,
    pub main_mint: String,
    pub secondary_mint: Option<String>,
}

impl PoolVaultInfoRaw {
    fn select_all_by_pool_registry(pool_registry: String) -> TypedQuery<Self> {
        query_as::<_, PoolVaultInfoRaw>("SELECT * FROM pool_vault_info WHERE pool_registry = $1")
            .bind(pool_registry)
    }

    fn select_all_by_main_mint(pool_registry: String, main_mint: String) -> TypedQuery<Self> {
        query_as::<_, PoolVaultInfoRaw>(
            "SELECT * FROM pool_vault_info WHERE pool_registry = $1 AND main_mint = $2",
        )
        .bind(pool_registry)
        .bind(main_mint)
    }

    fn insert(
        address: String,
        pool_registry: String,
        main_mint: String,
        secondary_mint: Option<String>,
    ) -> TypedQuery<Self> {
        query_as::<_, PoolVaultInfoRaw>(
            r#"INSERT INTO pool_vault_info
            (address, pool_registry, main_mint, secondary_mint)
            VALUES ($1, $2, $3, $4)
            RETURNING address, pool_registry, main_mint, secondary_mint"#,
        )
        .bind(address)
        .bind(pool_registry)
        .bind(main_mint)
        .bind(secondary_mint)
    }
}

impl Into<PoolVaultInfo> for PoolVaultInfoRaw {
    fn into(self) -> PoolVaultInfo {
        PoolVaultInfo {
            address: Pubkey::from_str(&self.address).unwrap(),
            pool_registry: Pubkey::from_str(&self.pool_registry).unwrap(),
            main_mint: Pubkey::from_str(&self.main_mint).unwrap(),
            secondary_mint: self.secondary_mint.map(|m| Pubkey::from_str(&m).unwrap()),
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoolVaultInfo {
    #[serde(with = "pubkey")]
    pub address: Pubkey,
    #[serde(with = "pubkey")]
    pub pool_registry: Pubkey,
    #[serde(with = "pubkey")]
    pub main_mint: Pubkey,
    #[serde(with = "option_pubkey")]
    pub secondary_mint: Option<Pubkey>,
}

impl Database {
    pub async fn select_pool_vault_info_by_pool_registry(
        &self,
        pool_registry: &Pubkey,
    ) -> Result<Vec<PoolVaultInfo>> {
        self.fetch_all(PoolVaultInfoRaw::select_all_by_pool_registry(
            pool_registry.to_string(),
        ))
        .await
    }

    pub async fn select_pool_vault_info_by_main_mint(
        &self,
        pool_registry: &Pubkey,
        main_mint: &Pubkey,
    ) -> Result<Vec<PoolVaultInfo>> {
        self.fetch_all(PoolVaultInfoRaw::select_all_by_main_mint(
            pool_registry.to_string(),
            main_mint.to_string(),
        ))
        .await
    }
    pub async fn insert_pool_vault_info(
        &self,
        address: &Pubkey,
        pool_registry: &Pubkey,
        main_mint: &Pubkey,
        secondary_mint: Option<&Pubkey>,
    ) -> Result<PoolVaultInfo> {
        self.fetch_one(PoolVaultInfoRaw::insert(
            address.to_string(),
            pool_registry.to_string(),
            main_mint.to_string(),
            secondary_mint.map(|m| m.to_string()),
        ))
        .await
    }
}
#[cfg(test)]
mod tests {
    use crate::utils::test_helpers::{connect_to_test_db, random_pubkey};

    #[tokio::test]
    async fn test_fee_vault_info() {
        let db = connect_to_test_db().await;
        let address1 = random_pubkey();
        let mint_a = random_pubkey();
        let mint_b = random_pubkey();
        let mint_c = random_pubkey();
        let pool_registry1 = random_pubkey();

        let info1 = db
            .insert_pool_vault_info(&address1, &pool_registry1, &mint_a, Some(&mint_b))
            .await
            .unwrap();

        let address2 = random_pubkey();

        let info2 = db
            .insert_pool_vault_info(&address2, &pool_registry1, &mint_a, Some(&mint_c))
            .await
            .unwrap();

        let address3 = random_pubkey();

        let info3 = db
            .insert_pool_vault_info(&address3, &pool_registry1, &mint_c, Some(&mint_a))
            .await
            .unwrap();

        let address4 = random_pubkey();
        let info4 = db
            .insert_pool_vault_info(&address4, &pool_registry1, &mint_c, None)
            .await
            .unwrap();

        let result1 = db
            .select_pool_vault_info_by_main_mint(&pool_registry1, &mint_a)
            .await
            .unwrap();
        assert!(result1.contains(&info1));
        assert!(result1.contains(&info2));
        assert!(!result1.contains(&info3));
        assert!(!result1.contains(&info4));

        let result2 = db
            .select_pool_vault_info_by_main_mint(&pool_registry1, &mint_c)
            .await
            .unwrap();
        assert!(!result2.contains(&info1));
        assert!(!result2.contains(&info2));
        assert!(result2.contains(&info3));
        assert!(result2.contains(&info4));

        let result3 = db
            .select_pool_vault_info_by_pool_registry(&pool_registry1)
            .await
            .unwrap();
        assert!(result3.contains(&info1));
        assert!(result3.contains(&info2));
        assert!(result3.contains(&info3));
        assert!(result3.contains(&info4));
    }
}
