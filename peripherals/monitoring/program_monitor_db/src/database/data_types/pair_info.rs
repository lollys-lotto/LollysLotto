use crate::{error::Result, utils::type_conversions::pubkey, Database, TypedQuery};
use lollys_lotto::Pair;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sqlx::query_as;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct PairInfoRaw {
    pub address: String,
    pub pool_registry: String,
    pub mint_a: String,
    pub mint_b: String,
}

impl PairInfoRaw {
    fn select_all_by_pool_registry(pool_registry: String) -> TypedQuery<Self> {
        query_as::<_, PairInfoRaw>("SELECT * FROM pair_info WHERE pool_registry = $1")
            .bind(pool_registry)
    }

    fn select_all_by_mint(pool_registry: String, mint: String) -> TypedQuery<Self> {
        query_as::<_, PairInfoRaw>(
            r#"SELECT * FROM pair_info
            WHERE pool_registry = $1 AND (mint_a = $2 OR mint_b = $2)"#,
        )
        .bind(pool_registry)
        .bind(mint)
    }

    fn insert(
        address: String,
        pool_registry: String,
        mint_a: String,
        mint_b: String,
    ) -> TypedQuery<Self> {
        query_as::<_, PairInfoRaw>(
            r#"INSERT INTO pair_info
            (address, pool_registry, mint_a, mint_b)
            VALUES ($1, $2, $3, $4)
            RETURNING address, pool_registry, mint_a, mint_b"#,
        )
        .bind(address)
        .bind(pool_registry)
        .bind(mint_a)
        .bind(mint_b)
    }
}

impl Into<PairInfo> for PairInfoRaw {
    fn into(self) -> PairInfo {
        let mint_a = Pubkey::from_str(&self.mint_a).unwrap();
        let mint_b = Pubkey::from_str(&self.mint_b).unwrap();
        let (mint_a, mint_b) = Pair::normalize_mint_order(mint_a, mint_b);
        PairInfo {
            address: Pubkey::from_str(&self.address).unwrap(),
            pool_registry: Pubkey::from_str(&self.pool_registry).unwrap(),
            mint_a,
            mint_b,
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PairInfo {
    #[serde(with = "pubkey")]
    pub address: Pubkey,
    #[serde(with = "pubkey")]
    pub pool_registry: Pubkey,
    #[serde(with = "pubkey")]
    pub mint_a: Pubkey,
    #[serde(with = "pubkey")]
    pub mint_b: Pubkey,
}

impl Database {
    pub async fn select_pair_info_by_pool_registry(
        &self,
        pool_registry: &Pubkey,
    ) -> Result<Vec<PairInfo>> {
        self.fetch_all(PairInfoRaw::select_all_by_pool_registry(
            pool_registry.to_string(),
        ))
        .await
    }

    pub async fn select_pair_info_by_mint(
        &self,
        pool_registry: &Pubkey,
        mint: &Pubkey,
    ) -> Result<Vec<PairInfo>> {
        self.fetch_all(PairInfoRaw::select_all_by_mint(
            pool_registry.to_string(),
            mint.to_string(),
        ))
        .await
    }
    pub async fn insert_pair_info(
        &self,
        pool_registry: &Pubkey,
        mint_a: &Pubkey,
        mint_b: &Pubkey,
    ) -> Result<PairInfo> {
        let (mint_a, mint_b) = Pair::normalize_mint_order(*mint_a, *mint_b);
        let address = Pair::address(*pool_registry, mint_a, mint_b);
        self.fetch_one(PairInfoRaw::insert(
            address.to_string(),
            pool_registry.to_string(),
            mint_a.to_string(),
            mint_b.to_string(),
        ))
        .await
    }
}
#[cfg(test)]
mod tests {
    use crate::utils::test_helpers::connect_to_test_db;
    use solana_sdk::{signature::Keypair, signer::Signer};

    #[tokio::test]
    async fn test_pair_info_info() {
        let db = connect_to_test_db().await;
        let pool_registry1 = Keypair::new().pubkey();
        let mint_a = Keypair::new().pubkey();
        let mint_b = Keypair::new().pubkey();
        let mint_c = Keypair::new().pubkey();

        let info1 = db
            .insert_pair_info(&pool_registry1, &mint_a, &mint_b)
            .await
            .unwrap();

        let info2 = db
            .insert_pair_info(&pool_registry1, &mint_a, &mint_c)
            .await
            .unwrap();

        let info3 = db
            .insert_pair_info(&pool_registry1, &mint_b, &mint_c)
            .await
            .unwrap();

        let pool_registry2 = Keypair::new().pubkey();
        let info4 = db
            .insert_pair_info(&pool_registry2, &mint_a, &mint_b)
            .await
            .unwrap();

        let result1 = db
            .select_pair_info_by_mint(&pool_registry1, &mint_a)
            .await
            .unwrap();
        assert!(result1.contains(&info1));
        assert!(result1.contains(&info2));
        assert!(!result1.contains(&info3));
        assert!(!result1.contains(&info4));

        let result2 = db
            .select_pair_info_by_mint(&pool_registry1, &mint_c)
            .await
            .unwrap();
        assert!(!result2.contains(&info1));
        assert!(result2.contains(&info2));
        assert!(result2.contains(&info3));
        assert!(!result2.contains(&info4));

        let result3 = db
            .select_pair_info_by_pool_registry(&pool_registry1)
            .await
            .unwrap();
        assert!(result3.contains(&info1));
        assert!(result3.contains(&info2));
        assert!(result3.contains(&info3));
        assert!(!result3.contains(&info4));
    }
}
