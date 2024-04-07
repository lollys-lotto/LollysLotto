use crate::{
    database::query_types::SlotRange,
    error::Result,
    utils::type_conversions::{
        byte_array_to_u64, naive_datetime_serde, pubkey, u64_to_postgres_types,
    },
    Database, DateTimeRange, TypedQuery,
};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sqlx::{
    query_as,
    types::{chrono::NaiveDateTime, Decimal},
};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct LottoGameVaultActivityRaw {
    pub event_id: i64,
    pub lotto_game: String,
    pub authority: String,
    pub lotto_game_vault: String,
    pub balance_before_num: Decimal,
    pub balance_before_arr: Vec<u8>,
    pub balance_after_num: Decimal,
    pub balance_after_arr: Vec<u8>,
    pub block_time: NaiveDateTime,
}

impl LottoGameVaultActivityRaw {
    fn select_all_after_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultActivityRaw>(
            r#"SELECT lgva.*, pe.block_time 
            FROM lotto_game_vault_activity as lgva 
            JOIN program_events as pe ON lgva.event_id = pe.id 
            WHERE lgva.lotto_game_vault = $1 AND pe.slot >= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_before_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultActivityRaw>(
            r#"SELECT lgva.*, pe.block_time 
            FROM lotto_game_vault_activity as lgva 
            JOIN program_events as pe ON lgva.event_id = pe.id 
            WHERE lgva.lotto_game_vault = $1 AND pe.slot <= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_in_slot_range(address: String, start: i64, end: i64) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultActivityRaw>(
            r#"SELECT lgva.*, pe.block_time 
            FROM lotto_game_vault_activity as lgva 
            JOIN program_events as pe ON lgva.event_id = pe.id 
            WHERE lgva.lotto_game_vault = $1 AND pe.slot >= $2 AND pe.slot <= $3
            "#,
        )
        .bind(address)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_slot(address: String, at: i64) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultActivityRaw>(
            r#"SELECT lgva.*, pe.block_time 
            FROM lotto_game_vault_activity as lgva 
            JOIN program_events as pe ON lgva.event_id = pe.id 
            WHERE lgva.lotto_game_vault = $1 AND pe.slot = $2
            "#,
        )
        .bind(address)
        .bind(at)
    }

    fn select_all_after_datetime(address: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultActivityRaw>(
            r#"SELECT lgva.*, pe.block_time 
            FROM lotto_game_vault_activity as lgva 
            JOIN program_events as pe ON lgva.event_id = pe.id 
            WHERE lgva.lotto_game_vault = $1 AND pe.block_time >= $2
            "#,
        )
        .bind(address)
        .bind(datetime)
    }

    fn select_all_before_datetime(address: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultActivityRaw>(
            r#"SELECT lgva.*, pe.block_time 
            FROM lotto_game_vault_activity as lgva 
            JOIN program_events as pe ON lgva.event_id = pe.id 
            WHERE lgva.lotto_game_vault = $1 AND pe.block_time <= $2
            "#,
        )
        .bind(address)
        .bind(datetime)
    }

    fn select_all_in_datetime_range(
        address: String,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultActivityRaw>(
            r#"SELECT lgva.*, pe.block_time 
            FROM lotto_game_vault_activity as lgva 
            JOIN program_events as pe ON lgva.event_id = pe.id 
            WHERE lgva.lotto_game_vault = $1 AND pe.block_time >= $2 AND pe.block_time <= $3
            "#,
        )
        .bind(address)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_datetime(address: String, at: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultActivityRaw>(
            r#"SELECT lgva.*, pe.block_time 
            FROM lotto_game_vault_activity as lgva 
            JOIN program_events as pe ON lgva.event_id = pe.id 
            WHERE lgva.lotto_game_vault = $1 AND pe.block_time = $2
            "#,
        )
        .bind(address)
        .bind(at)
    }

    fn insert(
        event_id: i64,
        lotto_game_vault: String,
        balance_before_num: Decimal,
        balance_before_arr: Vec<u8>,
        balance_after_num: Decimal,
        balance_after_arr: Vec<u8>,
    ) -> TypedQuery<Self> {
        query_as::<_, LottoGameVaultActivityRaw>(
            r#" WITH inserted AS (
                    INSERT INTO lotto_game_vault_activity (
                        event_id, 
                        lotto_game_vault, 
                        balance_before_num, 
                        balance_before_arr, 
                        balance_after_num, 
                        balance_after_arr
                    )
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING 
                        event_id, 
                        lotto_game_vault, 
                        balance_before_num, 
                        balance_before_arr, 
                        balance_after_num, 
                        balance_after_arr
                )
                SELECT
                    inserted.event_id,
                    inserted.lotto_game_vault,
                    inserted.balance_before_num,
                    inserted.balance_before_arr,
                    inserted.balance_after_num,
                    inserted.balance_after_arr,
                    pe.block_time
                FROM inserted
                JOIN program_events as pe ON inserted.event_id = pe.id
                "#,
        )
        .bind(event_id)
        .bind(lotto_game_vault)
        .bind(balance_before_num)
        .bind(balance_before_arr)
        .bind(balance_after_num)
        .bind(balance_after_arr)
    }
}

impl Into<LottoGameVaultActivity> for LottoGameVaultActivityRaw {
    fn into(self) -> LottoGameVaultActivity {
        let balance_before = byte_array_to_u64(&self.balance_before_arr).unwrap();
        let balance_after = byte_array_to_u64(&self.balance_after_arr).unwrap();

        LottoGameVaultActivity {
            event_id: self.event_id,
            lotto_game_vault: Pubkey::from_str(&self.lotto_game_vault).unwrap(),
            balance_before,
            balance_after,
            block_time: self.block_time,
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LottoGameVaultActivity {
    pub event_id: i64,
    #[serde(with = "pubkey")]
    pub lotto_game_vault: Pubkey,
    pub balance_before: u64,
    pub balance_after: u64,
    #[serde(with = "naive_datetime_serde")]
    pub block_time: NaiveDateTime,
}

impl Database {
    pub async fn select_lotto_game_vault_activity_in_slot_range(
        &self,
        lotto_game_vault: &Pubkey,
        slot_range: SlotRange,
    ) -> Result<Vec<LottoGameVaultActivity>> {
        let query = match slot_range {
            SlotRange::At(at) => {
                LottoGameVaultActivityRaw::select_all_at_slot(lotto_game_vault.to_string(), at)
            }
            SlotRange::Before(before) => LottoGameVaultActivityRaw::select_all_before_slot(
                lotto_game_vault.to_string(),
                before,
            ),
            SlotRange::After(after) => LottoGameVaultActivityRaw::select_all_after_slot(
                lotto_game_vault.to_string(),
                after,
            ),

            SlotRange::Between(start, end) => LottoGameVaultActivityRaw::select_all_in_slot_range(
                lotto_game_vault.to_string(),
                start,
                end,
            ),
        };
        self.fetch_all(query).await
    }

    pub async fn select_lotto_game_vault_activity_in_datetime_range(
        &self,
        lotto_game_vault: &Pubkey,
        datetime_range: DateTimeRange,
    ) -> Result<Vec<LottoGameVaultActivity>> {
        let query = match datetime_range {
            DateTimeRange::At(at) => {
                LottoGameVaultActivityRaw::select_all_at_datetime(lotto_game_vault.to_string(), at)
            }
            DateTimeRange::Before(before) => LottoGameVaultActivityRaw::select_all_before_datetime(
                lotto_game_vault.to_string(),
                before,
            ),
            DateTimeRange::After(after) => LottoGameVaultActivityRaw::select_all_after_datetime(
                lotto_game_vault.to_string(),
                after,
            ),
            DateTimeRange::Between(start, end) => {
                LottoGameVaultActivityRaw::select_all_in_datetime_range(
                    lotto_game_vault.to_string(),
                    start,
                    end,
                )
            }
        };
        self.fetch_all(query).await
    }

    pub async fn insert_lotto_game_vault_activity(
        &self,
        event_id: i64,
        lotto_game_vault: &Pubkey,
        balance_before: u64,
        balance_after: u64,
    ) -> Result<LottoGameVaultActivity> {
        let (balance_before_num, balance_before_arr) = u64_to_postgres_types(balance_before);
        let (balance_after_num, balance_after_arr) = u64_to_postgres_types(balance_after);
        self.fetch_one(LottoGameVaultActivityRaw::insert(
            event_id,
            lotto_game_vault.to_string(),
            balance_before_num,
            balance_before_arr,
            balance_after_num,
            balance_after_arr,
        ))
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_helpers::{connect_to_test_db, random_pubkey};

    #[tokio::test]
    async fn test_lotto_game_vault_activity() {
        let db = connect_to_test_db().await;
        let lotto_game_vault1 = random_pubkey();
        let lotto_game1 = random_pubkey();
        let authority1 = random_pubkey();
        let round1 = 1;

        let _info1 = db
            .insert_lotto_game_vault_info(&lotto_game_vault1, &lotto_game1, &authority1, round1)
            .await
            .unwrap();

        let lotto_game_vault2 = random_pubkey();
        let lotto_game2 = random_pubkey();
        let round2 = 2;

        let _info2 = db
            .insert_lotto_game_vault_info(&lotto_game_vault2, &lotto_game2, &authority1, round2)
            .await
            .unwrap();
    }
}
