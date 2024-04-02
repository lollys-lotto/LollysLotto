use crate::{
    database::{Database, TypedQuery},
    error::Result,
    utils::type_conversions::{option_naive_datetime_serde, option_pubkey, pubkey},
};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sqlx::{query_as, types::chrono::NaiveDateTime};
use std::str::FromStr;

/// Raw type for DB I/O
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct LiquidityAccountInfoRaw {
    pub address: String,
    pub owner: Option<String>,
    pub mint: String,
    pub pool_registry: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    /// Nullable because we might backfill this row while recording a deposit/withdraw etc.
    pub creation_event_id: Option<i64>,
    // TODO Method to populate this field on the correct row (address XXX, row with newest event id)
    /// Nullable because open accounts don't have a close event
    pub close_event_id: Option<i64>,
}

impl LiquidityAccountInfoRaw {
    fn select_by_owner(owner: &Pubkey) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountInfoRaw>(
            "SELECT * FROM liquidity_account_info WHERE owner = $1",
        )
        .bind(owner.to_string())
    }

    fn select_all_by_mint(mint: &Pubkey) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountInfoRaw>(
            "SELECT * FROM liquidity_account_info WHERE mint = $1",
        )
        .bind(mint.to_string())
    }

    fn update_close_event_id(
        close_event_id: i64,
        address: &Pubkey,
        created_at: NaiveDateTime,
    ) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountInfoRaw>(
            "UPDATE liquidity_account_info SET close_event_id = $1 WHERE address = $2 and created_at = $3 RETURNING *",
        )
        .bind(close_event_id)
        .bind(address.to_string())
        .bind(created_at)
    }

    fn insert(
        address: String,
        mint: String,
        owner: Option<String>,
        pool_registry: Option<String>,
        created_at: Option<NaiveDateTime>,
        creation_event_id: Option<i64>,
        close_event_id: Option<i64>,
    ) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountInfoRaw>(
            r#"INSERT INTO liquidity_account_info
            (address, owner, mint, pool_registry, created_at, creation_event_id, close_event_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING address, owner, mint, pool_registry, created_at, creation_event_id, close_event_id"#,
        )
            .bind(address)
            .bind(owner)
            .bind(mint)
            .bind(pool_registry)
            .bind(created_at)
            .bind(creation_event_id)
            .bind(close_event_id)
    }
}

impl Into<LiquidityAccountInfo> for LiquidityAccountInfoRaw {
    fn into(self) -> LiquidityAccountInfo {
        LiquidityAccountInfo {
            address: Pubkey::from_str(&self.address).unwrap(),
            owner: self.owner.map(|p| Pubkey::from_str(&p).unwrap()),
            mint: Pubkey::from_str(&self.mint).unwrap(),
            pool_registry: self.pool_registry.map(|p| Pubkey::from_str(&p).unwrap()),
            created_at: self.created_at,
            creation_event_id: self.creation_event_id,
            close_event_id: self.close_event_id,
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiquidityAccountInfo {
    #[serde(with = "pubkey")]
    pub address: Pubkey,
    #[serde(with = "pubkey")]
    pub mint: Pubkey,
    #[serde(with = "option_pubkey")]
    pub owner: Option<Pubkey>,
    #[serde(with = "option_pubkey")]
    pub pool_registry: Option<Pubkey>,
    #[serde(with = "option_naive_datetime_serde")]
    pub created_at: Option<NaiveDateTime>,
    pub creation_event_id: Option<i64>,
    pub close_event_id: Option<i64>,
}

impl Database {
    pub async fn select_liquidity_account_info_by_owner(
        &self,
        owner: &Pubkey,
    ) -> Result<Option<LiquidityAccountInfo>> {
        self.fetch_optional(LiquidityAccountInfoRaw::select_by_owner(owner))
            .await
    }

    pub async fn select_all_liquidity_accounts_by_mint(
        &self,
        mint: &Pubkey,
    ) -> Result<Vec<LiquidityAccountInfo>> {
        self.fetch_all(LiquidityAccountInfoRaw::select_all_by_mint(mint))
            .await
    }

    pub async fn update_liquidity_info_close_event_id(
        &self,
        close_event_id: i64,
        address: &Pubkey,
        created_at: NaiveDateTime,
    ) -> Result<Vec<LiquidityAccountInfo>> {
        let updated_rows = self
            .fetch_all(LiquidityAccountInfoRaw::update_close_event_id(
                close_event_id,
                address,
                created_at,
            ))
            .await?;

        // Convert updated rows from LiquidityAccountInfoRaw to LiquidityAccountInfo
        let updated_info = updated_rows
            .into_iter()
            .map(|raw: LiquidityAccountInfoRaw| raw.into())
            .collect();

        Ok(updated_info)
    }

    /// Insert a new row into the liquidity_account_info table.
    pub async fn insert_liquidity_account_info(
        &self,
        address: &Pubkey,
        mint: &Pubkey,
        owner: Option<&Pubkey>,
        pool_registry: Option<&Pubkey>,
        created_at: Option<NaiveDateTime>,
        creation_event_id: Option<i64>,
        close_event_id: Option<i64>,
    ) -> Result<LiquidityAccountInfo> {
        self.fetch_one(LiquidityAccountInfoRaw::insert(
            address.to_string(),
            mint.to_string(),
            owner.map(|owner| owner.to_string()),
            pool_registry.map(|pool_registry| pool_registry.to_string()),
            created_at,
            creation_event_id,
            close_event_id,
        ))
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        utils::test_helpers::{connect_to_test_db, insert_program_event, random_pubkey},
        InstructionType,
    };

    #[tokio::test]
    async fn liquidity_account_info_methods() {
        let db = connect_to_test_db().await;
        let owner = random_pubkey();
        let mint = random_pubkey();

        let program_event =
            insert_program_event(1, InstructionType::CreateLiquidityAccount, None, None, None)
                .await;

        let select_result = db
            .select_liquidity_account_info_by_owner(&owner)
            .await
            .unwrap();
        assert!(select_result.is_none());
        let insertion_result = db
            .insert_liquidity_account_info(
                &random_pubkey(),
                &mint,
                Some(&owner),
                Some(&random_pubkey()),
                Some(program_event.block_time),
                Some(program_event.id),
                None,
            )
            .await
            .unwrap();
        let select_result = db
            .select_liquidity_account_info_by_owner(&owner)
            .await
            .unwrap();
        assert_eq!(insertion_result, select_result.unwrap());
    }

    // TODO test select all by mint
}
