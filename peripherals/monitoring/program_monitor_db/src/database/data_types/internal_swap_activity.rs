use crate::{
    database::query_types::SlotRange,
    error::Result,
    utils::type_conversions::{
        byte_array_to_u128, naive_datetime_serde, pubkey, u128_to_postgres_types,
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
pub struct InternalSwapActivityRaw {
    pub event_id: i64,
    pub pair: String,
    pub token_a_price: f64,
    pub token_b_price: f64,
    pub ssl_a_internally_swapped_volume_num: Decimal,
    pub ssl_a_internally_swapped_volume_arr: Vec<u8>,
    pub ssl_b_internally_swapped_volume_num: Decimal,
    pub ssl_b_internally_swapped_volume_arr: Vec<u8>,
    pub block_time: NaiveDateTime,
}

impl InternalSwapActivityRaw {
    fn select_all_after_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, InternalSwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM internal_swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.slot >= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_before_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, InternalSwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM internal_swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.slot <= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_in_slot_range(address: String, start: i64, end: i64) -> TypedQuery<Self> {
        query_as::<_, InternalSwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM internal_swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.slot >= $2 AND e.slot <= $3
            "#,
        )
        .bind(address)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_slot(address: String, at: i64) -> TypedQuery<Self> {
        query_as::<_, InternalSwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM internal_swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.slot = $2
            "#,
        )
        .bind(address)
        .bind(at)
    }

    fn select_all_after_datetime(address: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, InternalSwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM internal_swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.block_time >= $2
            "#,
        )
        .bind(address)
        .bind(datetime)
    }

    fn select_all_before_datetime(address: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, InternalSwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM internal_swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.block_time <= $2
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
        query_as::<_, InternalSwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM internal_swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.block_time >= $2 AND e.block_time <= $3
            "#,
        )
        .bind(address)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_datetime(address: String, at: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, InternalSwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM internal_swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.block_time = $2
            "#,
        )
        .bind(address)
        .bind(at)
    }

    fn insert(
        event_id: i64,
        pair: String,
        token_a_price: f64,
        token_b_price: f64,
        ssl_a_internally_swapped_volume_num: Decimal,
        ssl_a_internally_swapped_volume_arr: Vec<u8>,
        ssl_b_internally_swapped_volume_num: Decimal,
        ssl_b_internally_swapped_volume_arr: Vec<u8>,
    ) -> TypedQuery<Self> {
        query_as::<_, InternalSwapActivityRaw>(
            r#"WITH inserted AS (
                    INSERT INTO internal_swap_activity
                    (
                        event_id,
                        pair,
                        token_a_price,
                        token_b_price,
                        ssl_a_internally_swapped_volume_num,
                        ssl_a_internally_swapped_volume_arr,
                        ssl_b_internally_swapped_volume_num,
                        ssl_b_internally_swapped_volume_arr
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING
                        event_id,
                        pair,
                        token_a_price,
                        token_b_price,
                        ssl_a_internally_swapped_volume_num,
                        ssl_a_internally_swapped_volume_arr,
                        ssl_b_internally_swapped_volume_num,
                        ssl_b_internally_swapped_volume_arr
                )
                SELECT
                    inserted.event_id,
                    inserted.pair,
                    inserted.token_a_price,
                    inserted.token_b_price,
                    inserted.ssl_a_internally_swapped_volume_num,
                    inserted.ssl_a_internally_swapped_volume_arr,
                    inserted.ssl_b_internally_swapped_volume_num,
                    inserted.ssl_b_internally_swapped_volume_arr,
                    e.block_time
                FROM inserted
                JOIN program_events AS e ON inserted.event_id = e.id
                "#,
        )
        .bind(event_id)
        .bind(pair)
        .bind(token_a_price)
        .bind(token_b_price)
        .bind(ssl_a_internally_swapped_volume_num)
        .bind(ssl_a_internally_swapped_volume_arr)
        .bind(ssl_b_internally_swapped_volume_num)
        .bind(ssl_b_internally_swapped_volume_arr)
    }
}

impl Into<InternalSwapActivity> for InternalSwapActivityRaw {
    fn into(self) -> InternalSwapActivity {
        let ssl_a_internally_swapped_volume =
            byte_array_to_u128(&self.ssl_a_internally_swapped_volume_arr).unwrap();
        let ssl_b_internally_swapped_volume =
            byte_array_to_u128(&self.ssl_b_internally_swapped_volume_arr).unwrap();
        InternalSwapActivity {
            event_id: self.event_id,
            pair: Pubkey::from_str(&self.pair).unwrap(),
            token_a_price: self.token_a_price,
            token_b_price: self.token_b_price,
            ssl_a_internally_swapped_volume,
            ssl_b_internally_swapped_volume,
            block_time: self.block_time,
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InternalSwapActivity {
    pub event_id: i64,
    #[serde(with = "pubkey")]
    pub pair: Pubkey,
    pub token_a_price: f64,
    pub token_b_price: f64,
    pub ssl_a_internally_swapped_volume: u128,
    pub ssl_b_internally_swapped_volume: u128,
    #[serde(with = "naive_datetime_serde")]
    pub block_time: NaiveDateTime,
}

impl Database {
    pub async fn select_internal_swap_activity_in_slot_range(
        &self,
        pair: &Pubkey,
        slot_range: SlotRange,
    ) -> Result<Vec<InternalSwapActivity>> {
        let query = match slot_range {
            SlotRange::At(at) => InternalSwapActivityRaw::select_all_at_slot(pair.to_string(), at),
            SlotRange::Before(before) => {
                InternalSwapActivityRaw::select_all_before_slot(pair.to_string(), before)
            }
            SlotRange::After(after) => {
                InternalSwapActivityRaw::select_all_after_slot(pair.to_string(), after)
            }
            SlotRange::Between(start, end) => {
                InternalSwapActivityRaw::select_all_in_slot_range(pair.to_string(), start, end)
            }
        };
        self.fetch_all(query).await
    }

    pub async fn select_internal_swap_activity_in_datetime_range(
        &self,
        pair: &Pubkey,
        datetime_range: DateTimeRange,
    ) -> Result<Vec<InternalSwapActivity>> {
        let query = match datetime_range {
            DateTimeRange::At(at) => {
                InternalSwapActivityRaw::select_all_at_datetime(pair.to_string(), at)
            }
            DateTimeRange::Before(before) => {
                InternalSwapActivityRaw::select_all_before_datetime(pair.to_string(), before)
            }
            DateTimeRange::After(after) => {
                InternalSwapActivityRaw::select_all_after_datetime(pair.to_string(), after)
            }
            DateTimeRange::Between(start, end) => {
                InternalSwapActivityRaw::select_all_in_datetime_range(pair.to_string(), start, end)
            }
        };
        self.fetch_all(query).await
    }

    pub async fn insert_internal_swap_activity(
        &self,
        event_id: i64,
        pair: &Pubkey,
        token_a_price: f64,
        token_b_price: f64,
        ssl_a_internally_swapped_volume: u128,
        ssl_b_internally_swapped_volume: u128,
    ) -> Result<InternalSwapActivity> {
        let (ssl_a_internally_swapped_volume_num, ssl_a_internally_swapped_volume_arr) =
            u128_to_postgres_types(ssl_a_internally_swapped_volume);
        let (ssl_b_internally_swapped_volume_num, ssl_b_internally_swapped_volume_arr) =
            u128_to_postgres_types(ssl_b_internally_swapped_volume);
        self.fetch_one(InternalSwapActivityRaw::insert(
            event_id,
            pair.to_string(),
            token_a_price,
            token_b_price,
            ssl_a_internally_swapped_volume_num,
            ssl_a_internally_swapped_volume_arr,
            ssl_b_internally_swapped_volume_num,
            ssl_b_internally_swapped_volume_arr,
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
    async fn test_internal_swap_activity() {
        let db = connect_to_test_db().await;
        let mint_a = random_pubkey();
        let mint_b = random_pubkey();
        let pool_registry = random_pubkey();

        let pair1 = db
            .insert_pair_info(&pool_registry, &mint_a, &mint_b)
            .await
            .unwrap();

        let _info2 = db
            .insert_pair_info(&pool_registry, &mint_a, &random_pubkey())
            .await
            .unwrap();

        let three_days_ago = now().checked_sub_days(Days::new(3)).unwrap();
        let two_days_ago = now().checked_sub_days(Days::new(2)).unwrap();
        let a_day_ago = now().checked_sub_days(Days::new(1)).unwrap();
        let event1 = insert_program_event(
            1000,
            InstructionType::InternalSwap,
            None,
            None,
            Some(three_days_ago),
        )
        .await;
        let activity1 = db
            .insert_internal_swap_activity(event1.id, &pair1.address, 0.1, 0.15, 1234, 4567)
            .await
            .unwrap();

        let event2 = insert_program_event(
            2000,
            InstructionType::InternalSwap,
            None,
            None,
            Some(two_days_ago),
        )
        .await;
        let activity2 = db
            .insert_internal_swap_activity(event2.id, &pair1.address, 0.2, 0.25, 1235, 4568)
            .await
            .unwrap();

        let event3 = insert_program_event(
            3000,
            InstructionType::InternalSwap,
            None,
            None,
            Some(a_day_ago),
        )
        .await;
        let activity3 = db
            .insert_internal_swap_activity(event3.id, &pair1.address, 0.3, 0.35, 1236, 4569)
            .await
            .unwrap();

        let event4 =
            insert_program_event(4000, InstructionType::InternalSwap, None, None, Some(now()))
                .await;
        let activity4 = db
            .insert_internal_swap_activity(event4.id, &pair1.address, 0.4, 0.45, 1237, 4570)
            .await
            .unwrap();

        let result = db
            .select_internal_swap_activity_in_slot_range(
                &pair1.address,
                SlotRange::At(event1.slot as i64),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_internal_swap_activity_in_slot_range(
                &pair1.address,
                SlotRange::Before(event1.slot as i64),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_internal_swap_activity_in_slot_range(
                &pair1.address,
                SlotRange::After(event2.slot as i64),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(result.contains(&activity4));

        let result = db
            .select_internal_swap_activity_in_slot_range(
                &pair1.address,
                SlotRange::Between(event2.slot as i64, event3.slot as i64),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_internal_swap_activity_in_datetime_range(
                &pair1.address,
                DateTimeRange::At(event1.block_time),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_internal_swap_activity_in_datetime_range(
                &pair1.address,
                DateTimeRange::Before(event1.block_time),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_internal_swap_activity_in_datetime_range(
                &pair1.address,
                DateTimeRange::After(event2.block_time),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(result.contains(&activity4));

        let result = db
            .select_internal_swap_activity_in_datetime_range(
                &pair1.address,
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
