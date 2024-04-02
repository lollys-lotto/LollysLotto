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
pub struct PoolVaultActivityRaw {
    pub event_id: i64,
    pub pool_vault: String,
    pub balance_before_num: Decimal,
    pub balance_before_arr: Vec<u8>,
    pub balance_after_num: Decimal,
    pub balance_after_arr: Vec<u8>,
    pub block_time: NaiveDateTime,
}

impl PoolVaultActivityRaw {
    fn select_all_after_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, PoolVaultActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM pool_vault_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_vault = $1 AND e.slot >= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_before_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, PoolVaultActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM pool_vault_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_vault = $1 AND e.slot <= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_in_slot_range(address: String, start: i64, end: i64) -> TypedQuery<Self> {
        query_as::<_, PoolVaultActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM pool_vault_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_vault = $1 AND e.slot >= $2 AND e.slot <= $3
            "#,
        )
        .bind(address)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_slot(address: String, at: i64) -> TypedQuery<Self> {
        query_as::<_, PoolVaultActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM pool_vault_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_vault = $1 AND e.slot = $2
            "#,
        )
        .bind(address)
        .bind(at)
    }

    fn select_all_after_datetime(address: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, PoolVaultActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM pool_vault_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_vault = $1 AND e.block_time >= $2
            "#,
        )
        .bind(address)
        .bind(datetime)
    }

    fn select_all_before_datetime(address: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, PoolVaultActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM pool_vault_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_vault = $1 AND e.block_time <= $2
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
        query_as::<_, PoolVaultActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM pool_vault_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_vault = $1 AND e.block_time >= $2 AND e.block_time <= $3
            "#,
        )
        .bind(address)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_datetime(address: String, at: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, PoolVaultActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM pool_vault_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_vault = $1 AND e.block_time = $2
            "#,
        )
        .bind(address)
        .bind(at)
    }

    fn insert(
        event_id: i64,
        pool_vault: String,
        balance_before_num: Decimal,
        balance_before_arr: Vec<u8>,
        balance_after_num: Decimal,
        balance_after_arr: Vec<u8>,
    ) -> TypedQuery<Self> {
        query_as::<_, PoolVaultActivityRaw>(
            r#"WITH inserted AS (
                    INSERT INTO pool_vault_activity (
                        event_id,
                        pool_vault,
                        balance_before_num,
                        balance_before_arr,
                        balance_after_num,
                        balance_after_arr
                    )
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING
                        event_id,
                        pool_vault,
                        balance_before_num,
                        balance_before_arr,
                        balance_after_num,
                        balance_after_arr
                )
                SELECT
                    inserted.event_id,
                    inserted.pool_vault,
                    inserted.balance_before_num,
                    inserted.balance_before_arr,
                    inserted.balance_after_num,
                    inserted.balance_after_arr,
                    e.block_time
                FROM inserted
                JOIN program_events AS e ON inserted.event_id = e.id
                "#,
        )
        .bind(event_id)
        .bind(pool_vault)
        .bind(balance_before_num)
        .bind(balance_before_arr)
        .bind(balance_after_num)
        .bind(balance_after_arr)
    }

    fn select_latest_activity_for_pool_vault(pool_vault_address: String) -> TypedQuery<Self> {
        query_as::<_, PoolVaultActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM pool_vault_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_vault = $1 AND e.error_message IS NULL
            ORDER BY e.slot DESC
            LIMIT 1
            "#,
        )
        .bind(pool_vault_address)
    }
}

impl Into<PoolVaultActivity> for PoolVaultActivityRaw {
    fn into(self) -> PoolVaultActivity {
        let balance_before = byte_array_to_u64(&self.balance_before_arr).unwrap();
        let balance_after = byte_array_to_u64(&self.balance_after_arr).unwrap();
        PoolVaultActivity {
            event_id: self.event_id,
            pool_vault: Pubkey::from_str(&self.pool_vault).unwrap(),
            balance_before,
            balance_after,
            block_time: self.block_time,
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoolVaultActivity {
    pub event_id: i64,
    #[serde(with = "pubkey")]
    pub pool_vault: Pubkey,
    pub balance_before: u64,
    pub balance_after: u64,
    #[serde(with = "naive_datetime_serde")]
    pub block_time: NaiveDateTime,
}

impl Database {
    pub async fn select_pool_vault_activity_in_slot_range(
        &self,
        pool_vault: &Pubkey,
        slot_range: SlotRange,
    ) -> Result<Vec<PoolVaultActivity>> {
        let query = match slot_range {
            SlotRange::At(at) => {
                PoolVaultActivityRaw::select_all_at_slot(pool_vault.to_string(), at)
            }
            SlotRange::Before(before) => {
                PoolVaultActivityRaw::select_all_before_slot(pool_vault.to_string(), before)
            }
            SlotRange::After(after) => {
                PoolVaultActivityRaw::select_all_after_slot(pool_vault.to_string(), after)
            }
            SlotRange::Between(start, end) => {
                PoolVaultActivityRaw::select_all_in_slot_range(pool_vault.to_string(), start, end)
            }
        };
        self.fetch_all(query).await
    }

    pub async fn select_pool_vault_activity_in_datetime_range(
        &self,
        pool_vault: &Pubkey,
        datetime_range: DateTimeRange,
    ) -> Result<Vec<PoolVaultActivity>> {
        let query = match datetime_range {
            DateTimeRange::At(at) => {
                PoolVaultActivityRaw::select_all_at_datetime(pool_vault.to_string(), at)
            }
            DateTimeRange::Before(before) => {
                PoolVaultActivityRaw::select_all_before_datetime(pool_vault.to_string(), before)
            }
            DateTimeRange::After(after) => {
                PoolVaultActivityRaw::select_all_after_datetime(pool_vault.to_string(), after)
            }
            DateTimeRange::Between(start, end) => {
                PoolVaultActivityRaw::select_all_in_datetime_range(
                    pool_vault.to_string(),
                    start,
                    end,
                )
            }
        };
        self.fetch_all(query).await
    }

    pub async fn insert_pool_vault_activity(
        &self,
        event_id: i64,
        pool_vault: &Pubkey,
        balance_before: u64,
        balance_after: u64,
    ) -> Result<PoolVaultActivity> {
        let (balance_before_num, balance_before_arr) = u64_to_postgres_types(balance_before);
        let (balance_after_num, balance_after_arr) = u64_to_postgres_types(balance_after);
        self.fetch_one(PoolVaultActivityRaw::insert(
            event_id,
            pool_vault.to_string(),
            balance_before_num,
            balance_before_arr,
            balance_after_num,
            balance_after_arr,
        ))
        .await
    }

    pub async fn fetch_latest_pool_vault_activity_by_mint(
        &self,
        pool_vault: &Pubkey,
    ) -> Result<PoolVaultActivity> {
        let query =
            PoolVaultActivityRaw::select_latest_activity_for_pool_vault(pool_vault.to_string());
        self.fetch_one(query).await
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        error::ForeignKey,
        utils::test_helpers::{connect_to_test_db, insert_program_event, now, random_pubkey},
        DateTimeRange, InstructionType, SSLv2DatabaseError, SlotRange,
    };
    use chrono::Days;

    #[tokio::test]
    async fn test_pool_vault_activity() {
        let db = connect_to_test_db().await;
        let address1 = random_pubkey();
        let mint1 = random_pubkey();
        let mint2 = random_pubkey();
        let pool_registry1 = random_pubkey();

        let _info1 = db
            .insert_pool_vault_info(&address1, &pool_registry1, &mint1, Some(&mint2))
            .await
            .unwrap();

        let address2 = random_pubkey();
        let mint2 = random_pubkey();

        let _info2 = db
            .insert_pool_vault_info(&address2, &pool_registry1, &mint2, Some(&mint1))
            .await
            .unwrap();

        let three_days_ago = now().checked_sub_days(Days::new(3)).unwrap();
        let two_days_ago = now().checked_sub_days(Days::new(2)).unwrap();
        let a_day_ago = now().checked_sub_days(Days::new(1)).unwrap();
        let event1 = insert_program_event(
            1000,
            InstructionType::Deposit,
            None,
            None,
            Some(three_days_ago),
        )
        .await;
        let activity1 = db
            .insert_pool_vault_activity(event1.id, &address1, 1234, 2345)
            .await
            .unwrap();
        assert_ne!(activity1.balance_before, activity1.balance_after);

        let event2 = insert_program_event(
            2000,
            InstructionType::Deposit,
            None,
            None,
            Some(two_days_ago),
        )
        .await;
        let activity2 = db
            .insert_pool_vault_activity(event2.id, &address1, 1235, 2346)
            .await
            .unwrap();

        let event3 =
            insert_program_event(3000, InstructionType::Deposit, None, None, Some(a_day_ago)).await;
        let activity3 = db
            .insert_pool_vault_activity(event3.id, &address1, 1236, 2347)
            .await
            .unwrap();

        let event4 =
            insert_program_event(4000, InstructionType::Deposit, None, None, Some(now())).await;
        let activity4 = db
            .insert_pool_vault_activity(event4.id, &address1, 1237, 2348)
            .await
            .unwrap();

        let result = db
            .select_pool_vault_activity_in_slot_range(&address1, SlotRange::At(event1.slot as i64))
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_pool_vault_activity_in_slot_range(
                &address1,
                SlotRange::Before(event1.slot as i64),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_pool_vault_activity_in_slot_range(
                &address1,
                SlotRange::After(event2.slot as i64),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(result.contains(&activity4));

        let result = db
            .select_pool_vault_activity_in_slot_range(
                &address1,
                SlotRange::Between(event2.slot as i64, event3.slot as i64),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_pool_vault_activity_in_datetime_range(
                &address1,
                DateTimeRange::At(event1.block_time),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_pool_vault_activity_in_datetime_range(
                &address1,
                DateTimeRange::Before(event1.block_time),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_pool_vault_activity_in_datetime_range(
                &address1,
                DateTimeRange::After(event2.block_time),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(result.contains(&activity4));

        let result = db
            .select_pool_vault_activity_in_datetime_range(
                &address1,
                DateTimeRange::Between(event2.block_time, event3.block_time),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(!result.contains(&activity4));
    }

    #[tokio::test]
    async fn foreign_key_constraints() {
        let db = connect_to_test_db().await;

        let program_event =
            insert_program_event(1, InstructionType::CreateLiquidityAccount, None, None, None)
                .await;

        // Trying to insert this activity with no corresponding pool vault should fail
        // with a foreign key constraint for table `ForeignKey::PoolVault`.
        match db
            .insert_pool_vault_activity(program_event.id, &random_pubkey(), 1234, 2345)
            .await
        {
            Ok(_) => panic!("Failed to raise foreign key violation"),
            Err(e) => match e {
                SSLv2DatabaseError::MissingForeignKey(fk, _) => {
                    assert_eq!(ForeignKey::PoolVault, fk);
                }
                _ => panic!("Failed to raise correct SSLv2 DB error variant, {:?}", e),
            },
        }
    }
}
