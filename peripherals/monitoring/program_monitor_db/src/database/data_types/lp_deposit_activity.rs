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
pub struct LpDepositActivityRaw {
    pub event_id: i64,
    pub pool_registry: String,
    pub mint: String,
    pub total_liquidity_deposited_num: Decimal,
    pub total_liquidity_deposited_arr: Vec<u8>,
    pub block_time: NaiveDateTime,
}

impl LpDepositActivityRaw {
    fn select_all_after_slot(pool_registry: String, mint: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM lp_deposit_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_registry = $1 AND p.mint = $2 AND e.slot >= $3
            "#,
        )
        .bind(pool_registry)
        .bind(mint)
        .bind(slot)
    }

    fn select_all_before_slot(pool_registry: String, mint: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM lp_deposit_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_registry = $1 AND p.mint = $2 AND e.slot <= $3
            "#,
        )
        .bind(pool_registry)
        .bind(mint)
        .bind(slot)
    }

    fn select_all_in_slot_range(
        pool_registry: String,
        mint: String,
        start: i64,
        end: i64,
    ) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM lp_deposit_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_registry = $1 AND p.mint = $2 AND e.slot >= $3 AND e.slot <= $4
            "#,
        )
        .bind(pool_registry)
        .bind(mint)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_slot(pool_registry: String, mint: String, at: i64) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM lp_deposit_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_registry = $1 AND p.mint = $2 AND e.slot = $3
            "#,
        )
        .bind(pool_registry)
        .bind(mint)
        .bind(at)
    }

    fn select_all_after_datetime(
        pool_registry: String,
        mint: String,
        datetime: NaiveDateTime,
    ) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM lp_deposit_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_registry = $1 AND p.mint = $2 AND e.block_time >= $3
            "#,
        )
        .bind(pool_registry)
        .bind(mint)
        .bind(datetime)
    }

    fn select_all_before_datetime(
        pool_registry: String,
        mint: String,
        datetime: NaiveDateTime,
    ) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM lp_deposit_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_registry = $1 AND p.mint = $2 AND e.block_time <= $3
            "#,
        )
        .bind(pool_registry)
        .bind(mint)
        .bind(datetime)
    }

    fn select_all_in_datetime_range(
        pool_registry: String,
        mint: String,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM lp_deposit_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_registry = $1 AND p.mint = $2 AND e.block_time >= $3 AND e.block_time <= $4
            "#,
        )
        .bind(pool_registry)
        .bind(mint)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_datetime(
        pool_registry: String,
        mint: String,
        at: NaiveDateTime,
    ) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM lp_deposit_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pool_registry = $1 AND p.mint = $2 AND e.block_time = $3
            "#,
        )
        .bind(pool_registry)
        .bind(mint)
        .bind(at)
    }

    fn insert(
        event_id: i64,
        pool_registry: String,
        mint: String,
        total_liquidity_deposited_num: Decimal,
        total_liquidity_deposited_arr: Vec<u8>,
    ) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"WITH inserted AS (
                    INSERT INTO lp_deposit_activity
                    (event_id, pool_registry, mint, total_liquidity_deposited_num, total_liquidity_deposited_arr)
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING event_id, pool_registry, mint, total_liquidity_deposited_num, total_liquidity_deposited_arr
                )
                SELECT inserted.event_id, inserted.pool_registry, inserted.mint, inserted.total_liquidity_deposited_num, inserted.total_liquidity_deposited_arr, e.block_time
                FROM inserted
                JOIN program_events AS e ON inserted.event_id = e.id
                "#,
        )
            .bind(event_id)
            .bind(pool_registry)
            .bind(mint)
            .bind(total_liquidity_deposited_num)
            .bind(total_liquidity_deposited_arr)
    }

    fn select_latest_lp_deposit_activity_by_mint(mint: String) -> TypedQuery<Self> {
        query_as::<_, LpDepositActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM lp_deposit_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.mint = $1 AND e.error_message IS NULL
            ORDER BY e.slot DESC
            LIMIT 1
            "#,
        )
        .bind(mint)
    }
}

impl Into<LpDepositActivity> for LpDepositActivityRaw {
    fn into(self) -> LpDepositActivity {
        let total_liquidity_deposited =
            byte_array_to_u64(&self.total_liquidity_deposited_arr).unwrap();
        LpDepositActivity {
            pool_registry: Pubkey::from_str(&self.pool_registry).unwrap(),
            mint: Pubkey::from_str(&self.mint).unwrap(),
            event_id: self.event_id,
            total_liquidity_deposited,
            block_time: self.block_time,
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LpDepositActivity {
    pub event_id: i64,
    #[serde(with = "pubkey")]
    pub pool_registry: Pubkey,
    #[serde(with = "pubkey")]
    pub mint: Pubkey,
    pub total_liquidity_deposited: u64,
    #[serde(with = "naive_datetime_serde")]
    pub block_time: NaiveDateTime,
}

impl Database {
    pub async fn select_lp_deposit_activity_in_slot_range(
        &self,
        pool_registry: &Pubkey,
        mint: &Pubkey,
        slot_range: SlotRange,
    ) -> Result<Vec<LpDepositActivity>> {
        let query = match slot_range {
            SlotRange::At(at) => LpDepositActivityRaw::select_all_at_slot(
                pool_registry.to_string(),
                mint.to_string(),
                at,
            ),
            SlotRange::Before(before) => LpDepositActivityRaw::select_all_before_slot(
                pool_registry.to_string(),
                mint.to_string(),
                before,
            ),
            SlotRange::After(after) => LpDepositActivityRaw::select_all_after_slot(
                pool_registry.to_string(),
                mint.to_string(),
                after,
            ),
            SlotRange::Between(start, end) => LpDepositActivityRaw::select_all_in_slot_range(
                pool_registry.to_string(),
                mint.to_string(),
                start,
                end,
            ),
        };
        self.fetch_all(query).await
    }

    pub async fn select_lp_deposit_activity_in_datetime_range(
        &self,
        pool_registry: &Pubkey,
        mint: &Pubkey,
        datetime_range: DateTimeRange,
    ) -> Result<Vec<LpDepositActivity>> {
        let query = match datetime_range {
            DateTimeRange::At(at) => LpDepositActivityRaw::select_all_at_datetime(
                pool_registry.to_string(),
                mint.to_string(),
                at,
            ),
            DateTimeRange::Before(before) => LpDepositActivityRaw::select_all_before_datetime(
                pool_registry.to_string(),
                mint.to_string(),
                before,
            ),
            DateTimeRange::After(after) => LpDepositActivityRaw::select_all_after_datetime(
                pool_registry.to_string(),
                mint.to_string(),
                after,
            ),
            DateTimeRange::Between(start, end) => {
                LpDepositActivityRaw::select_all_in_datetime_range(
                    pool_registry.to_string(),
                    mint.to_string(),
                    start,
                    end,
                )
            }
        };
        self.fetch_all(query).await
    }

    pub async fn insert_lp_deposit_activity(
        &self,
        event_id: i64,
        pool_registry: &Pubkey,
        mint: &Pubkey,
        total_liquidity_deposited: u64,
    ) -> Result<LpDepositActivity> {
        let (total_liquidity_deposited_num, total_liquidity_deposited_arr) =
            u64_to_postgres_types(total_liquidity_deposited);
        self.fetch_one(LpDepositActivityRaw::insert(
            event_id,
            pool_registry.to_string(),
            mint.to_string(),
            total_liquidity_deposited_num,
            total_liquidity_deposited_arr,
        ))
        .await
    }

    pub async fn fetch_latest_lp_deposit_vault_activity_by_mint(
        &self,
        mint: &Pubkey,
    ) -> Result<LpDepositActivity> {
        let query =
            LpDepositActivityRaw::select_latest_lp_deposit_activity_by_mint(mint.to_string());
        self.fetch_one(query).await
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        utils::test_helpers::{connect_to_test_db, insert_program_event, now, random_pubkey},
        DateTimeRange, InstructionType, SlotRange,
    };
    use chrono::Days;

    #[tokio::test]
    async fn test_lp_deposit_activity() {
        let db = connect_to_test_db().await;
        let address1 = random_pubkey();
        let pool_registry = random_pubkey();

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
            .insert_lp_deposit_activity(event1.id, &pool_registry, &address1, 1234)
            .await
            .unwrap();

        let event2 = insert_program_event(
            2000,
            InstructionType::Deposit,
            None,
            None,
            Some(two_days_ago),
        )
        .await;
        let activity2 = db
            .insert_lp_deposit_activity(event2.id, &pool_registry, &address1, 1235)
            .await
            .unwrap();

        let event3 =
            insert_program_event(3000, InstructionType::Deposit, None, None, Some(a_day_ago)).await;
        let activity3 = db
            .insert_lp_deposit_activity(event3.id, &pool_registry, &address1, 1236)
            .await
            .unwrap();

        let event4 =
            insert_program_event(4000, InstructionType::Deposit, None, None, Some(now())).await;
        let activity4 = db
            .insert_lp_deposit_activity(event4.id, &pool_registry, &address1, 1237)
            .await
            .unwrap();

        let result = db
            .select_lp_deposit_activity_in_slot_range(
                &pool_registry,
                &address1,
                SlotRange::At(event1.slot as i64),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_lp_deposit_activity_in_slot_range(
                &pool_registry,
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
            .select_lp_deposit_activity_in_slot_range(
                &pool_registry,
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
            .select_lp_deposit_activity_in_slot_range(
                &pool_registry,
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
            .select_lp_deposit_activity_in_datetime_range(
                &pool_registry,
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
            .select_lp_deposit_activity_in_datetime_range(
                &pool_registry,
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
            .select_lp_deposit_activity_in_datetime_range(
                &pool_registry,
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
            .select_lp_deposit_activity_in_datetime_range(
                &pool_registry,
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
}
