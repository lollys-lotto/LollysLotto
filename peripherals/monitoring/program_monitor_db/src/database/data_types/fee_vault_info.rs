use crate::{error::Result, utils::type_conversions::pubkey, Database, TypedQuery};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sqlx::query_as;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct FeeVaultInfoRaw {
    pub address: String,
    pub pool_registry: String,
    pub mint: String,
}

impl FeeVaultInfoRaw {
    fn select_all_by_pool_registry(pool_registry: &Pubkey) -> TypedQuery<Self> {
        query_as::<_, FeeVaultInfoRaw>("SELECT * FROM fee_vault_info WHERE pool_registry = $1")
            .bind(pool_registry.to_string())
    }

    fn select_by_mint(mint: &Pubkey) -> TypedQuery<Self> {
        query_as::<_, FeeVaultInfoRaw>("SELECT * FROM fee_vault_info WHERE mint = $1")
            .bind(mint.to_string())
    }

    fn insert(address: String, pool_registry: String, mint: String) -> TypedQuery<Self> {
        query_as::<_, FeeVaultInfoRaw>(
            r#"INSERT INTO fee_vault_info
            (address, pool_registry, mint)
            VALUES ($1, $2, $3)
            RETURNING address, pool_registry, mint"#,
        )
        .bind(address)
        .bind(pool_registry)
        .bind(mint)
    }
}

impl Into<FeeVaultInfo> for FeeVaultInfoRaw {
    fn into(self) -> FeeVaultInfo {
        FeeVaultInfo {
            address: Pubkey::from_str(&self.address).unwrap(),
            pool_registry: Pubkey::from_str(&self.pool_registry).unwrap(),
            mint: Pubkey::from_str(&self.mint).unwrap(),
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeVaultInfo {
    #[serde(with = "pubkey")]
    pub address: Pubkey,
    #[serde(with = "pubkey")]
    pub pool_registry: Pubkey,
    #[serde(with = "pubkey")]
    pub mint: Pubkey,
}

impl Database {
    pub async fn select_fee_vault_info_by_pool_registry(
        &self,
        pool_registry: &Pubkey,
    ) -> Result<Vec<FeeVaultInfo>> {
        self.fetch_all(FeeVaultInfoRaw::select_all_by_pool_registry(pool_registry))
            .await
    }

    pub async fn select_fee_vault_info_by_mint(
        &self,
        mint: &Pubkey,
    ) -> Result<Option<FeeVaultInfo>> {
        self.fetch_optional(FeeVaultInfoRaw::select_by_mint(mint))
            .await
    }

    pub async fn insert_fee_vault_info(
        &self,
        address: &Pubkey,
        pool_registry: &Pubkey,
        mint: &Pubkey,
    ) -> Result<FeeVaultInfo> {
        self.fetch_one(FeeVaultInfoRaw::insert(
            address.to_string(),
            pool_registry.to_string(),
            mint.to_string(),
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
        let mint1 = random_pubkey();
        let pool_registry1 = random_pubkey();

        let info1 = db
            .insert_fee_vault_info(&address1, &pool_registry1, &mint1)
            .await
            .unwrap();

        let address2 = random_pubkey();
        let mint2 = random_pubkey();

        let info2 = db
            .insert_fee_vault_info(&address2, &pool_registry1, &mint2)
            .await
            .unwrap();

        let result1 = db
            .select_fee_vault_info_by_mint(&mint1)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(info1, result1);
        assert_eq!(mint1, result1.mint);
        assert_eq!(pool_registry1, result1.pool_registry);

        let result2 = db
            .select_fee_vault_info_by_mint(&mint2)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(info2, result2);

        let result3 = db
            .select_fee_vault_info_by_pool_registry(&pool_registry1)
            .await
            .unwrap();
        assert!(result3.contains(&result1));
        assert!(result3.contains(&result2));
    }
}
