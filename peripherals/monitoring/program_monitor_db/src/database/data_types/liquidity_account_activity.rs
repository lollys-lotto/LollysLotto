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
pub struct LiquidityAccountActivityRaw {
    pub event_id: i64,
    pub liquidity_account: String,
    pub amount_deposited_num: Decimal,
    pub amount_deposited_arr: Vec<u8>,
    pub last_observed_tap_num: Decimal,
    pub last_observed_tap_arr: Vec<u8>,
    pub total_earned_num: Decimal,
    pub total_earned_arr: Vec<u8>,
    pub block_time: NaiveDateTime,
}

impl LiquidityAccountActivityRaw {
    fn select_all_after_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM liquidity_account_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.liquidity_account = $1 AND e.slot >= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_before_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM liquidity_account_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.liquidity_account = $1 AND e.slot <= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_in_slot_range(address: String, start: i64, end: i64) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM liquidity_account_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.liquidity_account = $1 AND e.slot >= $2 AND e.slot <= $3
            "#,
        )
        .bind(address)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_slot(address: String, at: i64) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM liquidity_account_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.liquidity_account = $1 AND e.slot = $2
            "#,
        )
        .bind(address)
        .bind(at)
    }

    fn select_all_after_datetime(address: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM liquidity_account_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.liquidity_account = $1 AND e.block_time >= $2
            "#,
        )
        .bind(address)
        .bind(datetime)
    }

    fn select_all_before_datetime(address: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM liquidity_account_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.liquidity_account = $1 AND e.block_time <= $2
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
        query_as::<_, LiquidityAccountActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM liquidity_account_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.liquidity_account = $1 AND e.block_time >= $2 AND e.block_time <= $3
            "#,
        )
        .bind(address)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_datetime(address: String, at: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM liquidity_account_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.liquidity_account = $1 AND e.block_time = $2
            "#,
        )
        .bind(address)
        .bind(at)
    }

    fn insert(
        liquidity_account: String,
        event_id: i64,
        amount_deposited_num: Decimal,
        amount_deposited_arr: Vec<u8>,
        last_observed_tap_num: Decimal,
        last_observed_tap_arr: Vec<u8>,
        total_earned_num: Decimal,
        total_earned_arr: Vec<u8>,
    ) -> TypedQuery<Self> {
        query_as::<_, LiquidityAccountActivityRaw>(
            r#"WITH inserted AS (
                    INSERT INTO liquidity_account_activity
                    (
                        event_id,
                        liquidity_account,
                        amount_deposited_num,
                        amount_deposited_arr,
                        last_observed_tap_num,
                        last_observed_tap_arr,
                        total_earned_num,
                        total_earned_arr
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING
                        event_id,
                        liquidity_account,
                        amount_deposited_num,
                        amount_deposited_arr,
                        last_observed_tap_num,
                        last_observed_tap_arr,
                        total_earned_num,
                        total_earned_arr
                )
                SELECT
                    inserted.event_id,
                    inserted.liquidity_account,
                    inserted.amount_deposited_num,
                    inserted.amount_deposited_arr,
                    inserted.last_observed_tap_num,
                    inserted.last_observed_tap_arr,
                    inserted.total_earned_num,
                    inserted.total_earned_arr,
                    e.block_time
                FROM inserted
                JOIN program_events AS e ON inserted.event_id = e.id
                "#,
        )
        .bind(event_id)
        .bind(liquidity_account)
        .bind(amount_deposited_num)
        .bind(amount_deposited_arr)
        .bind(last_observed_tap_num)
        .bind(last_observed_tap_arr)
        .bind(total_earned_num)
        .bind(total_earned_arr)
    }
}

impl Into<LiquidityAccountActivity> for LiquidityAccountActivityRaw {
    fn into(self) -> LiquidityAccountActivity {
        let amount_deposited = byte_array_to_u64(&self.amount_deposited_arr).unwrap();
        let last_observed_tap = byte_array_to_u64(&self.last_observed_tap_arr).unwrap();
        let total_earned = byte_array_to_u64(&self.total_earned_arr).unwrap();
        LiquidityAccountActivity {
            event_id: self.event_id,
            liquidity_account: Pubkey::from_str(&self.liquidity_account).unwrap(),
            amount_deposited,
            last_observed_tap,
            total_earned,
            block_time: self.block_time,
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiquidityAccountActivity {
    pub event_id: i64,
    #[serde(with = "pubkey")]
    pub liquidity_account: Pubkey,
    pub amount_deposited: u64,
    pub last_observed_tap: u64,
    pub total_earned: u64,
    #[serde(with = "naive_datetime_serde")]
    pub block_time: NaiveDateTime,
}

impl Database {
    pub async fn select_liquidity_account_activity_in_slot_range(
        &self,
        liquidity_account: &Pubkey,
        slot_range: SlotRange,
    ) -> Result<Vec<LiquidityAccountActivity>> {
        let query = match slot_range {
            SlotRange::At(at) => {
                LiquidityAccountActivityRaw::select_all_at_slot(liquidity_account.to_string(), at)
            }
            SlotRange::Before(before) => LiquidityAccountActivityRaw::select_all_before_slot(
                liquidity_account.to_string(),
                before,
            ),
            SlotRange::After(after) => LiquidityAccountActivityRaw::select_all_after_slot(
                liquidity_account.to_string(),
                after,
            ),
            SlotRange::Between(start, end) => {
                LiquidityAccountActivityRaw::select_all_in_slot_range(
                    liquidity_account.to_string(),
                    start,
                    end,
                )
            }
        };
        self.fetch_all(query).await
    }

    pub async fn select_liquidity_account_activity_in_datetime_range(
        &self,
        liquidity_account: &Pubkey,
        datetime_range: DateTimeRange,
    ) -> Result<Vec<LiquidityAccountActivity>> {
        let query = match datetime_range {
            DateTimeRange::At(at) => LiquidityAccountActivityRaw::select_all_at_datetime(
                liquidity_account.to_string(),
                at,
            ),
            DateTimeRange::Before(before) => {
                LiquidityAccountActivityRaw::select_all_before_datetime(
                    liquidity_account.to_string(),
                    before,
                )
            }
            DateTimeRange::After(after) => LiquidityAccountActivityRaw::select_all_after_datetime(
                liquidity_account.to_string(),
                after,
            ),
            DateTimeRange::Between(start, end) => {
                LiquidityAccountActivityRaw::select_all_in_datetime_range(
                    liquidity_account.to_string(),
                    start,
                    end,
                )
            }
        };
        self.fetch_all(query).await
    }

    pub async fn insert_liquidity_account_activity(
        &self,
        event_id: i64,
        liquidity_account: &Pubkey,
        amount_deposited: u64,
        last_observed_tap: u64,
        total_earned: u64,
    ) -> Result<LiquidityAccountActivity> {
        let (amount_deposited_num, amount_deposited_arr) = u64_to_postgres_types(amount_deposited);
        let (last_observed_tap_num, last_observed_tap_arr) =
            u64_to_postgres_types(last_observed_tap);
        let (total_earned_num, total_earned_arr) = u64_to_postgres_types(total_earned);
        self.fetch_one(LiquidityAccountActivityRaw::insert(
            liquidity_account.to_string(),
            event_id,
            amount_deposited_num,
            amount_deposited_arr,
            last_observed_tap_num,
            last_observed_tap_arr,
            total_earned_num,
            total_earned_arr,
        ))
        .await
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
    async fn test_liquidity_account_activity() {
        let db = connect_to_test_db().await;
        let address1 = random_pubkey();
        let mint = random_pubkey();
        let pool_registry = random_pubkey();

        let _info1 = db
            .insert_liquidity_account_info(
                &address1,
                &mint,
                Some(&random_pubkey()),
                Some(&pool_registry),
                Some(now()),
                None,
                None,
            )
            .await
            .unwrap();

        let address2 = random_pubkey();

        let _info2 = db
            .insert_liquidity_account_info(
                &address2,
                &mint,
                Some(&random_pubkey()),
                Some(&pool_registry),
                Some(now()),
                None,
                None,
            )
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
            .insert_liquidity_account_activity(event1.id, &address1, 1234, 2345, 3456)
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
            .insert_liquidity_account_activity(event2.id, &address1, 1235, 2346, 3457)
            .await
            .unwrap();

        let event3 =
            insert_program_event(3000, InstructionType::Deposit, None, None, Some(a_day_ago)).await;
        let activity3 = db
            .insert_liquidity_account_activity(event3.id, &address1, 1236, 2347, 3458)
            .await
            .unwrap();

        let event4 =
            insert_program_event(4000, InstructionType::Deposit, None, None, Some(now())).await;
        let activity4 = db
            .insert_liquidity_account_activity(event4.id, &address1, 1237, 2348, 3459)
            .await
            .unwrap();

        let result = db
            .select_liquidity_account_activity_in_slot_range(
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
            .select_liquidity_account_activity_in_slot_range(
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
            .select_liquidity_account_activity_in_slot_range(
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
            .select_liquidity_account_activity_in_slot_range(
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
            .select_liquidity_account_activity_in_datetime_range(
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
            .select_liquidity_account_activity_in_datetime_range(
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
            .select_liquidity_account_activity_in_datetime_range(
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
            .select_liquidity_account_activity_in_datetime_range(
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
