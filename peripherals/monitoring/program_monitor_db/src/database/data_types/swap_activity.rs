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

// TODO Add more queries here, based around user ID, mint_in or mint_out
// TODO Add queries that JOIN on pool vault and fee vault activity

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct SwapActivityRaw {
    pub event_id: i64,
    pub pair: String,
    pub user: String,
    pub mint_in: String,
    pub mint_out: String,
    pub amount_in_num: Decimal,
    pub amount_in_arr: Vec<u8>,
    pub preswap_pool_token_ratio: f64,
    pub initially_below_max_pool_token_ratio: bool,
    pub can_take_preferred_route: bool,
    pub violates_pool_token_ratio_constraints: bool,
    pub optimal_price: f64,
    pub panic_price: f64,
    pub amount_out_optimal_num: Decimal,
    pub amount_out_optimal_arr: Vec<u8>,
    pub fees_out_optimal_num: Decimal,
    pub fees_out_optimal_arr: Vec<u8>,
    pub amount_out_panic_num: Decimal,
    pub amount_out_panic_arr: Vec<u8>,
    pub fees_out_panic_num: Decimal,
    pub fees_out_panic_arr: Vec<u8>,
    pub total_accumulated_lp_rewards_num: Decimal,
    pub total_accumulated_lp_rewards_arr: Vec<u8>,
    pub block_time: NaiveDateTime,
}

impl SwapActivityRaw {
    fn select_all_after_slot(pair: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, SwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.slot >= $2
            "#,
        )
        .bind(pair)
        .bind(slot)
    }

    fn select_all_before_slot(pair: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, SwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.slot <= $2
            "#,
        )
        .bind(pair)
        .bind(slot)
    }

    fn select_all_in_slot_range(pair: String, start: i64, end: i64) -> TypedQuery<Self> {
        query_as::<_, SwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.slot >= $2 AND e.slot <= $3
            "#,
        )
        .bind(pair)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_slot(pair: String, at: i64) -> TypedQuery<Self> {
        query_as::<_, SwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.slot = $2
            "#,
        )
        .bind(pair)
        .bind(at)
    }

    fn select_all_after_datetime(pair: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, SwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.block_time >= $2
            "#,
        )
        .bind(pair)
        .bind(datetime)
    }

    fn select_all_before_datetime(pair: String, datetime: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, SwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.block_time <= $2
            "#,
        )
        .bind(pair)
        .bind(datetime)
    }

    fn select_all_in_datetime_range(
        pair: String,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> TypedQuery<Self> {
        query_as::<_, SwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.block_time >= $2 AND e.block_time <= $3
            "#,
        )
        .bind(pair)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_datetime(pair: String, at: NaiveDateTime) -> TypedQuery<Self> {
        query_as::<_, SwapActivityRaw>(
            r#"SELECT p.*, e.block_time
            FROM swap_activity AS p
            JOIN program_events AS e ON p.event_id = e.id
            WHERE p.pair = $1 AND e.block_time = $2
            "#,
        )
        .bind(pair)
        .bind(at)
    }

    fn insert(
        event_id: i64,
        pair: String,
        user: String,
        mint_in: String,
        mint_out: String,
        amount_in_num: Decimal,
        amount_in_arr: Vec<u8>,
        preswap_pool_token_ratio: f64,
        initially_below_max_pool_token_ratio: bool,
        can_take_preferred_route: bool,
        violates_pool_token_ratio_constraints: bool,
        optimal_price: f64,
        panic_price: f64,
        amount_out_optimal_num: Decimal,
        amount_out_optimal_arr: Vec<u8>,
        fees_out_optimal_num: Decimal,
        fees_out_optimal_arr: Vec<u8>,
        amount_out_panic_num: Decimal,
        amount_out_panic_arr: Vec<u8>,
        fees_out_panic_num: Decimal,
        fees_out_panic_arr: Vec<u8>,
        total_accumulated_lp_rewards_num: Decimal,
        total_accumulated_lp_rewards_arr: Vec<u8>,
    ) -> TypedQuery<Self> {
        query_as::<_, SwapActivityRaw>(
            r#"WITH inserted AS (
                    INSERT INTO swap_activity
                    (
                        event_id,
                        pair,
                        "user",
                        mint_in,
                        mint_out,
                        amount_in_num,
                        amount_in_arr,
                        preswap_pool_token_ratio,
                        initially_below_max_pool_token_ratio,
                        can_take_preferred_route,
                        violates_pool_token_ratio_constraints,
                        optimal_price,
                        panic_price,
                        amount_out_optimal_num,
                        amount_out_optimal_arr,
                        fees_out_optimal_num,
                        fees_out_optimal_arr,
                        amount_out_panic_num,
                        amount_out_panic_arr,
                        fees_out_panic_num,
                        fees_out_panic_arr,
                        total_accumulated_lp_rewards_num,
                        total_accumulated_lp_rewards_arr
                        )
                    VALUES (
                        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
                        $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23
                    )
                    RETURNING
                        event_id,
                        pair,
                        "user",
                        mint_in,
                        mint_out,
                        amount_in_num,
                        amount_in_arr,
                        preswap_pool_token_ratio,
                        initially_below_max_pool_token_ratio,
                        can_take_preferred_route,
                        violates_pool_token_ratio_constraints,
                        optimal_price,
                        panic_price,
                        amount_out_optimal_num,
                        amount_out_optimal_arr,
                        fees_out_optimal_num,
                        fees_out_optimal_arr,
                        amount_out_panic_num,
                        amount_out_panic_arr,
                        fees_out_panic_num,
                        fees_out_panic_arr,
                        total_accumulated_lp_rewards_num,
                        total_accumulated_lp_rewards_arr
                )
                SELECT
                    inserted.event_id,
                    inserted.pair,
                    inserted.user,
                    inserted.mint_in,
                    inserted.mint_out,
                    inserted.amount_in_num,
                    inserted.amount_in_arr,
                    inserted.preswap_pool_token_ratio,
                    inserted.initially_below_max_pool_token_ratio,
                    inserted.can_take_preferred_route,
                    inserted.violates_pool_token_ratio_constraints,
                    inserted.optimal_price,
                    inserted.panic_price,
                    inserted.amount_out_optimal_num,
                    inserted.amount_out_optimal_arr,
                    inserted.fees_out_optimal_num,
                    inserted.fees_out_optimal_arr,
                    inserted.amount_out_panic_num,
                    inserted.amount_out_panic_arr,
                    inserted.fees_out_panic_num,
                    inserted.fees_out_panic_arr,
                    inserted.total_accumulated_lp_rewards_num,
                    inserted.total_accumulated_lp_rewards_arr,
                    e.block_time
                FROM inserted
                JOIN program_events AS e ON inserted.event_id = e.id
                "#,
        )
        .bind(event_id)
        .bind(pair)
        .bind(user)
        .bind(mint_in)
        .bind(mint_out)
        .bind(amount_in_num)
        .bind(amount_in_arr)
        .bind(preswap_pool_token_ratio)
        .bind(initially_below_max_pool_token_ratio)
        .bind(can_take_preferred_route)
        .bind(violates_pool_token_ratio_constraints)
        .bind(optimal_price)
        .bind(panic_price)
        .bind(amount_out_optimal_num)
        .bind(amount_out_optimal_arr)
        .bind(fees_out_optimal_num)
        .bind(fees_out_optimal_arr)
        .bind(amount_out_panic_num)
        .bind(amount_out_panic_arr)
        .bind(fees_out_panic_num)
        .bind(fees_out_panic_arr)
        .bind(total_accumulated_lp_rewards_num)
        .bind(total_accumulated_lp_rewards_arr)
    }
}

impl Into<SwapActivity> for SwapActivityRaw {
    fn into(self) -> SwapActivity {
        let amount_in = byte_array_to_u64(&self.amount_in_arr).unwrap();
        let amount_out_optimal = byte_array_to_u64(&self.amount_out_optimal_arr).unwrap();
        let fees_out_optimal = byte_array_to_u64(&self.fees_out_optimal_arr).unwrap();
        let amount_out_panic = byte_array_to_u64(&self.amount_out_panic_arr).unwrap();
        let fees_out_panic = byte_array_to_u64(&self.fees_out_panic_arr).unwrap();
        let total_accumulated_lp_rewards =
            byte_array_to_u64(&self.total_accumulated_lp_rewards_arr).unwrap();
        SwapActivity {
            event_id: self.event_id,
            pair: Pubkey::from_str(&self.pair).unwrap(),
            user: Pubkey::from_str(&self.user).unwrap(),
            mint_in: Pubkey::from_str(&self.mint_in).unwrap(),
            mint_out: Pubkey::from_str(&self.mint_out).unwrap(),
            amount_in,
            preswap_pool_token_ratio: self.preswap_pool_token_ratio,
            initially_below_max_pool_token_ratio: self.initially_below_max_pool_token_ratio,
            can_take_preferred_route: self.can_take_preferred_route,
            violates_pool_token_ratio_constraints: self.violates_pool_token_ratio_constraints,
            optimal_price: self.optimal_price,
            panic_price: self.panic_price,
            amount_out_optimal,
            fees_out_optimal,
            amount_out_panic,
            fees_out_panic,
            total_accumulated_lp_rewards,
            block_time: self.block_time,
        }
    }
}

/// Type-converted for ease-of-use
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwapActivity {
    pub event_id: i64,
    #[serde(with = "pubkey")]
    pub pair: Pubkey,
    #[serde(with = "pubkey")]
    pub user: Pubkey,
    #[serde(with = "pubkey")]
    pub mint_in: Pubkey,
    #[serde(with = "pubkey")]
    pub mint_out: Pubkey,
    pub amount_in: u64,
    pub preswap_pool_token_ratio: f64,
    pub initially_below_max_pool_token_ratio: bool,
    pub can_take_preferred_route: bool,
    pub violates_pool_token_ratio_constraints: bool,
    pub optimal_price: f64,
    pub panic_price: f64,
    pub amount_out_optimal: u64,
    pub fees_out_optimal: u64,
    pub amount_out_panic: u64,
    pub fees_out_panic: u64,
    pub total_accumulated_lp_rewards: u64,
    #[serde(with = "naive_datetime_serde")]
    pub block_time: NaiveDateTime,
}

impl Database {
    pub async fn select_swap_activity_in_slot_range(
        &self,
        pair: &Pubkey,
        slot_range: SlotRange,
    ) -> Result<Vec<SwapActivity>> {
        let query = match slot_range {
            SlotRange::At(at) => SwapActivityRaw::select_all_at_slot(pair.to_string(), at),
            SlotRange::Before(before) => {
                SwapActivityRaw::select_all_before_slot(pair.to_string(), before)
            }
            SlotRange::After(after) => {
                SwapActivityRaw::select_all_after_slot(pair.to_string(), after)
            }
            SlotRange::Between(start, end) => {
                SwapActivityRaw::select_all_in_slot_range(pair.to_string(), start, end)
            }
        };
        self.fetch_all(query).await
    }

    pub async fn select_swap_activity_in_datetime_range(
        &self,
        pair: &Pubkey,
        datetime_range: DateTimeRange,
    ) -> Result<Vec<SwapActivity>> {
        let query = match datetime_range {
            DateTimeRange::At(at) => SwapActivityRaw::select_all_at_datetime(pair.to_string(), at),
            DateTimeRange::Before(before) => {
                SwapActivityRaw::select_all_before_datetime(pair.to_string(), before)
            }
            DateTimeRange::After(after) => {
                SwapActivityRaw::select_all_after_datetime(pair.to_string(), after)
            }
            DateTimeRange::Between(start, end) => {
                SwapActivityRaw::select_all_in_datetime_range(pair.to_string(), start, end)
            }
        };
        self.fetch_all(query).await
    }

    pub async fn insert_swap_activity(
        &self,
        event_id: i64,
        pair: &Pubkey,
        user: &Pubkey,
        mint_in: &Pubkey,
        mint_out: &Pubkey,
        amount_in: u64,
        preswap_pool_token_ratio: f64,
        initially_below_max_pool_token_ratio: bool,
        can_take_preferred_route: bool,
        violates_pool_token_ratio_constraints: bool,
        optimal_price: f64,
        panic_price: f64,
        amount_out_optimal: u64,
        fees_out_optimal: u64,
        amount_out_panic: u64,
        fees_out_panic: u64,
        total_accumulated_lp_rewards: u64,
    ) -> Result<SwapActivity> {
        let (amount_in_num, amount_in_arr) = u64_to_postgres_types(amount_in);
        let (amount_out_optimal_num, amount_out_optimal_arr) =
            u64_to_postgres_types(amount_out_optimal);
        let (fees_out_optimal_num, fees_out_optimal_arr) = u64_to_postgres_types(fees_out_optimal);
        let (amount_out_panic_num, amount_out_panic_arr) = u64_to_postgres_types(amount_out_panic);
        let (fees_out_panic_num, fees_out_panic_arr) = u64_to_postgres_types(fees_out_panic);
        let (total_accumulated_lp_rewards_num, total_accumulated_lp_rewards_arr) =
            u64_to_postgres_types(total_accumulated_lp_rewards);
        self.fetch_one(SwapActivityRaw::insert(
            event_id,
            pair.to_string(),
            user.to_string(),
            mint_in.to_string(),
            mint_out.to_string(),
            amount_in_num,
            amount_in_arr,
            preswap_pool_token_ratio,
            initially_below_max_pool_token_ratio,
            can_take_preferred_route,
            violates_pool_token_ratio_constraints,
            optimal_price,
            panic_price,
            amount_out_optimal_num,
            amount_out_optimal_arr,
            fees_out_optimal_num,
            fees_out_optimal_arr,
            amount_out_panic_num,
            amount_out_panic_arr,
            fees_out_panic_num,
            fees_out_panic_arr,
            total_accumulated_lp_rewards_num,
            total_accumulated_lp_rewards_arr,
        ))
        .await
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
    async fn test_swap_activity() {
        let db = connect_to_test_db().await;
        let mint = random_pubkey();
        let pool_registry = random_pubkey();

        let info1 = db
            .insert_pair_info(&pool_registry, &random_pubkey(), &mint)
            .await
            .unwrap();

        let _info2 = db
            .insert_pair_info(&pool_registry, &random_pubkey(), &mint)
            .await
            .unwrap();

        let three_days_ago = now().checked_sub_days(Days::new(3)).unwrap();
        let two_days_ago = now().checked_sub_days(Days::new(2)).unwrap();
        let a_day_ago = now().checked_sub_days(Days::new(1)).unwrap();
        let event1 = insert_program_event(
            1000,
            InstructionType::Swap,
            None,
            None,
            Some(three_days_ago),
        )
        .await;
        let activity1 = db
            .insert_swap_activity(
                event1.id,
                &info1.address,
                &random_pubkey(),
                &info1.mint_a,
                &info1.mint_b,
                1234,
                0.01,
                false,
                true,
                false,
                234.5,
                236.5,
                12,
                1,
                14,
                1,
                100,
            )
            .await
            .unwrap();

        let event2 =
            insert_program_event(2000, InstructionType::Swap, None, None, Some(two_days_ago)).await;
        let activity2 = db
            .insert_swap_activity(
                event2.id,
                &info1.address,
                &random_pubkey(),
                &info1.mint_a,
                &info1.mint_b,
                1235,
                0.02,
                false,
                true,
                false,
                234.5,
                236.5,
                13,
                2,
                15,
                2,
                101,
            )
            .await
            .unwrap();

        let event3 =
            insert_program_event(3000, InstructionType::Swap, None, None, Some(a_day_ago)).await;
        let activity3 = db
            .insert_swap_activity(
                event3.id,
                &info1.address,
                &random_pubkey(),
                &info1.mint_a,
                &info1.mint_b,
                1236,
                0.03,
                false,
                true,
                false,
                234.6,
                236.6,
                15,
                3,
                16,
                3,
                102,
            )
            .await
            .unwrap();

        let event4 =
            insert_program_event(4000, InstructionType::Swap, None, None, Some(now())).await;
        let activity4 = db
            .insert_swap_activity(
                event4.id,
                &info1.address,
                &random_pubkey(),
                &info1.mint_a,
                &info1.mint_b,
                1237,
                0.04,
                false,
                true,
                false,
                234.7,
                236.7,
                16,
                4,
                17,
                4,
                103,
            )
            .await
            .unwrap();

        let result = db
            .select_swap_activity_in_slot_range(&info1.address, SlotRange::At(event1.slot as i64))
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_swap_activity_in_slot_range(
                &info1.address,
                SlotRange::Before(event1.slot as i64),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_swap_activity_in_slot_range(
                &info1.address,
                SlotRange::After(event2.slot as i64),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(result.contains(&activity4));

        let result = db
            .select_swap_activity_in_slot_range(
                &info1.address,
                SlotRange::Between(event2.slot as i64, event3.slot as i64),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_swap_activity_in_datetime_range(
                &info1.address,
                DateTimeRange::At(event1.block_time),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_swap_activity_in_datetime_range(
                &info1.address,
                DateTimeRange::Before(event1.block_time),
            )
            .await
            .unwrap();
        assert!(result.contains(&activity1));
        assert!(!result.contains(&activity2));
        assert!(!result.contains(&activity3));
        assert!(!result.contains(&activity4));

        let result = db
            .select_swap_activity_in_datetime_range(
                &info1.address,
                DateTimeRange::After(event2.block_time),
            )
            .await
            .unwrap();
        assert!(!result.contains(&activity1));
        assert!(result.contains(&activity2));
        assert!(result.contains(&activity3));
        assert!(result.contains(&activity4));

        let result = db
            .select_swap_activity_in_datetime_range(
                &info1.address,
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

        let program_event = insert_program_event(1, InstructionType::Swap, None, None, None).await;

        // Trying to insert this activity with no corresponding liquidity account should fail
        // with a foreign key constraint for table `ForeignKey::LiquidityAccount`.
        match db
            .insert_swap_activity(
                program_event.id,
                &random_pubkey(),
                &random_pubkey(),
                &random_pubkey(),
                &random_pubkey(),
                1237,
                0.04,
                false,
                true,
                false,
                234.7,
                236.7,
                16,
                4,
                17,
                4,
                103,
            )
            .await
        {
            Ok(_) => panic!("Failed to raise foreign key violation"),
            Err(e) => match e {
                SSLv2DatabaseError::MissingForeignKey(fk, _) => {
                    assert_eq!(ForeignKey::Pair, fk);
                }
                _ => panic!("Failed to raise correct SSLv2 DB error variant, {:?}", e),
            },
        }
    }
}
