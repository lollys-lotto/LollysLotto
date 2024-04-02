use crate::{error::Result, utils::type_conversions::pubkey, Database, TypedQuery};
use lollys_lotto::{OraclePriceHistory, OracleType};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sqlx::query_as;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct OraclePriceHistoryInfoRaw {
    pub address: String,
    pub pool_registry: String,
    pub oracle: String,
    pub oracle_type: OracleType,
    pub mint: String,
}

impl OraclePriceHistoryInfoRaw {
    fn select_all_by_pool_registry(pool_registry: String) -> TypedQuery<Self> {
        query_as::<_, OraclePriceHistoryInfoRaw>(
            "SELECT * FROM oracle_price_history_info WHERE pool_registry = $1",
        )
        .bind(pool_registry)
    }

    fn select_all_by_mint(pool_registry: String, mint: String) -> TypedQuery<Self> {
        query_as::<_, OraclePriceHistoryInfoRaw>(
            r#"SELECT * FROM oracle_price_history_info
            WHERE pool_registry = $1 AND mint = $2"#,
        )
        .bind(pool_registry)
        .bind(mint)
    }

    fn select_all_by_mint_and_oracle(
        pool_registry: String,
        mint: String,
        oracle: String,
    ) -> TypedQuery<Self> {
        query_as::<_, OraclePriceHistoryInfoRaw>(
            r#"SELECT * FROM oracle_price_history_info
            WHERE pool_registry = $1 AND mint = $2 AND oracle = $3"#,
        )
        .bind(pool_registry)
        .bind(mint)
        .bind(oracle)
    }

    fn insert(
        address: String,
        pool_registry: String,
        oracle: String,
        oracle_type: OracleType,
        mint: String,
    ) -> TypedQuery<Self> {
        query_as::<_, OraclePriceHistoryInfoRaw>(
            r#"INSERT INTO oracle_price_history_info
            (address, pool_registry, oracle, oracle_type, mint)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING address, pool_registry, oracle, oracle_type, mint"#,
        )
        .bind(address)
        .bind(pool_registry)
        .bind(oracle)
        .bind(oracle_type)
        .bind(mint)
    }
}

impl Into<OraclePriceHistoryInfo> for OraclePriceHistoryInfoRaw {
    fn into(self) -> OraclePriceHistoryInfo {
        OraclePriceHistoryInfo {
            address: Pubkey::from_str(&self.address).unwrap(),
            pool_registry: Pubkey::from_str(&self.pool_registry).unwrap(),
            oracle: Pubkey::from_str(&self.oracle).unwrap(),
            oracle_type: self.oracle_type,
            mint: Pubkey::from_str(&self.mint).unwrap(),
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OraclePriceHistoryInfo {
    #[serde(with = "pubkey")]
    pub address: Pubkey,
    #[serde(with = "pubkey")]
    pub pool_registry: Pubkey,
    #[serde(with = "pubkey")]
    pub oracle: Pubkey,
    pub oracle_type: OracleType,
    #[serde(with = "pubkey")]
    pub mint: Pubkey,
}

impl Database {
    pub async fn select_oracle_price_history_info_by_pool_registry(
        &self,
        pool_registry: &Pubkey,
    ) -> Result<Vec<OraclePriceHistoryInfo>> {
        self.fetch_all(OraclePriceHistoryInfoRaw::select_all_by_pool_registry(
            pool_registry.to_string(),
        ))
        .await
    }

    pub async fn select_oracle_price_history_info_by_mint(
        &self,
        pool_registry: &Pubkey,
        mint: &Pubkey,
    ) -> Result<Vec<OraclePriceHistoryInfo>> {
        self.fetch_all(OraclePriceHistoryInfoRaw::select_all_by_mint(
            pool_registry.to_string(),
            mint.to_string(),
        ))
        .await
    }

    pub async fn select_oracle_price_history_info(
        &self,
        pool_registry: &Pubkey,
        mint: &Pubkey,
        oracle: &Pubkey,
    ) -> Result<Option<OraclePriceHistoryInfo>> {
        self.fetch_optional(OraclePriceHistoryInfoRaw::select_all_by_mint_and_oracle(
            pool_registry.to_string(),
            mint.to_string(),
            oracle.to_string(),
        ))
        .await
    }
    pub async fn insert_oracle_price_history_info(
        &self,
        pool_registry: &Pubkey,
        oracle: &Pubkey,
        oracle_type: OracleType,
        mint: &Pubkey,
    ) -> Result<OraclePriceHistoryInfo> {
        let address = OraclePriceHistory::address(pool_registry, oracle);
        self.fetch_one(OraclePriceHistoryInfoRaw::insert(
            address.to_string(),
            pool_registry.to_string(),
            oracle.to_string(),
            oracle_type,
            mint.to_string(),
        ))
        .await
    }
}
#[cfg(test)]
mod tests {
    use crate::utils::test_helpers::{connect_to_test_db, random_pubkey};
    use lollys_lotto::OracleType;
    use solana_sdk::{signature::Keypair, signer::Signer};

    #[tokio::test]
    async fn test_oracle_price_history_info() {
        let db = connect_to_test_db().await;
        let pool_registry1 = Keypair::new().pubkey();
        let oracle = Keypair::new().pubkey();
        let mint = Keypair::new().pubkey();
        let mint_c = Keypair::new().pubkey();

        let result = db
            .select_oracle_price_history_info(&pool_registry1, &oracle, &mint)
            .await
            .unwrap();
        assert!(result.is_none());

        let info1 = db
            .insert_oracle_price_history_info(&pool_registry1, &oracle, OracleType::Pyth, &mint)
            .await
            .unwrap();

        let result = db
            .select_oracle_price_history_info(&pool_registry1, &mint, &oracle)
            .await
            .unwrap();
        assert_eq!(info1, result.unwrap());

        let info2 = db
            .insert_oracle_price_history_info(
                &pool_registry1,
                &random_pubkey(),
                OracleType::Pyth,
                &mint_c,
            )
            .await
            .unwrap();

        let info3 = db
            .insert_oracle_price_history_info(
                &pool_registry1,
                &random_pubkey(),
                OracleType::Pyth,
                &mint_c,
            )
            .await
            .unwrap();

        let pool_registry2 = Keypair::new().pubkey();
        let info4 = db
            .insert_oracle_price_history_info(
                &pool_registry2,
                &random_pubkey(),
                OracleType::Pyth,
                &mint,
            )
            .await
            .unwrap();

        let result = db
            .select_oracle_price_history_info_by_mint(&pool_registry1, &mint)
            .await
            .unwrap();
        assert!(result.contains(&info1));
        assert!(!result.contains(&info2));
        assert!(!result.contains(&info3));
        assert!(!result.contains(&info4));

        let result = db
            .select_oracle_price_history_info_by_mint(&pool_registry2, &mint)
            .await
            .unwrap();
        assert!(!result.contains(&info1));
        assert!(!result.contains(&info2));
        assert!(!result.contains(&info3));
        assert!(result.contains(&info4));

        let result = db
            .select_oracle_price_history_info_by_mint(&pool_registry1, &mint_c)
            .await
            .unwrap();
        assert!(!result.contains(&info1));
        assert!(result.contains(&info2));
        assert!(result.contains(&info3));
        assert!(!result.contains(&info4));

        let result = db
            .select_oracle_price_history_info_by_pool_registry(&pool_registry1)
            .await
            .unwrap();
        assert!(result.contains(&info1));
        assert!(result.contains(&info2));
        assert!(result.contains(&info3));
        assert!(!result.contains(&info4));
    }
}
