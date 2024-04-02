use crate::{
    utils::type_conversions::{i64_to_u64, pubkey, u64_to_i64},
    Database, Result, SlotRange, TypedQuery, UntypedQuery,
};
use lollys_lotto::{HistoricalDecimal, HistoricalPrice};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sqlx::{query, query_as};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct OraclePriceRaw {
    pub oracle_price_history: String,
    pub price_num: i64,
    pub price_scale: i64,
    pub slot: i64,
}

impl OraclePriceRaw {
    fn select_all_after_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, OraclePriceRaw>(
            r#"SELECT p.*
            FROM oracle_prices AS p
            WHERE p.oracle_price_history = $1 AND p.slot >= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_before_slot(address: String, slot: i64) -> TypedQuery<Self> {
        query_as::<_, OraclePriceRaw>(
            r#"SELECT p.*
            FROM oracle_prices AS p
            WHERE p.oracle_price_history = $1 AND p.slot <= $2
            "#,
        )
        .bind(address)
        .bind(slot)
    }

    fn select_all_in_slot_range(address: String, start: i64, end: i64) -> TypedQuery<Self> {
        query_as::<_, OraclePriceRaw>(
            r#"SELECT p.*
            FROM oracle_prices AS p
            WHERE p.oracle_price_history = $1 AND p.slot >= $2 AND p.slot <= $3
            "#,
        )
        .bind(address)
        .bind(start)
        .bind(end)
    }

    fn select_all_at_slot(address: String, at: i64) -> TypedQuery<Self> {
        query_as::<_, OraclePriceRaw>(
            r#"SELECT p.*
            FROM oracle_prices AS p
            WHERE p.oracle_price_history = $1 AND p.slot = $2
            "#,
        )
        .bind(address)
        .bind(at)
    }

    pub fn insert(
        oracle_price_history: String,
        price_num: i64,
        price_scale: i64,
        slot: i64,
    ) -> TypedQuery<Self> {
        query_as::<_, OraclePriceRaw>(
            r#"
            INSERT INTO oracle_prices
            (oracle_price_history, price_num, price_scale, slot)
            VALUES ($1, $2, $3, $4)
            RETURNING oracle_price_history, price_num, price_scale, slot"#,
        )
        .bind(oracle_price_history)
        .bind(price_num)
        .bind(price_scale)
        .bind(slot)
    }

    pub fn insert_many(
        oracle_price_history: String,
        price_nums: Vec<i64>,
        price_scales: Vec<i64>,
        slots: Vec<i64>,
    ) -> UntypedQuery {
        query(
            r#"
            INSERT INTO oracle_prices
            (oracle_price_history, price_num, price_scale, slot)
            SELECT $1, fields.num, fields.scale, fields.slot
            FROM UNNEST($2::INT8[], $3::INT8[], $4::INT8[]) as fields(num, scale, slot)
            "#,
        )
        .bind(oracle_price_history)
        .bind(price_nums)
        .bind(price_scales)
        .bind(slots)
    }

    fn latest_entry(oracle_price_history: String) -> TypedQuery<Self> {
        query_as::<_, Self>(
            r#"
        SELECT * FROM oracle_prices
            WHERE slot = (
                SELECT MAX(slot)
                FROM oracle_prices
                WHERE oracle_price_history = $1
            )
        "#,
        )
        .bind(oracle_price_history)
    }

    pub fn latest_entry_by_mint(mint: String) -> TypedQuery<Self> {
        query_as::<_, Self>(
            r#"
            SELECT p.slot, p.price_num, p.price_scale, p.oracle_price_history
            FROM oracle_prices AS p
            INNER JOIN oracle_price_history_info AS o
            ON p.oracle_price_history = o.address
            WHERE o.mint = $1
            ORDER BY p.slot DESC
            LIMIT 1
            "#,
        )
        .bind(mint)
    }
}

impl Into<OraclePrice> for OraclePriceRaw {
    fn into(self) -> OraclePrice {
        OraclePrice {
            oracle_price_history: Pubkey::from_str(&self.oracle_price_history).unwrap(),
            price_num: self.price_num,
            price_scale: self.price_scale as u32,
            slot: i64_to_u64(self.slot),
        }
    }
}

impl Into<HistoricalPrice> for OraclePriceRaw {
    fn into(self) -> HistoricalPrice {
        HistoricalPrice {
            price: HistoricalDecimal {
                num: self.price_num,
                scale: self.price_scale as u32,
                _pad0: [0u8; 4],
            },
            slot: i64_to_u64(self.slot),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OraclePrice {
    #[serde(with = "pubkey")]
    pub oracle_price_history: Pubkey,
    pub price_num: i64,
    pub price_scale: u32,
    pub slot: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OraclePriceWithMint {
    #[serde(with = "pubkey")]
    pub oracle_price_history: Pubkey,
    #[serde(with = "pubkey")]
    pub mint: Pubkey,
    pub price_num: i64,
    pub price_scale: u32,
    pub slot: u64,
}

impl Into<HistoricalPrice> for OraclePrice {
    fn into(self) -> HistoricalPrice {
        HistoricalPrice {
            price: HistoricalDecimal {
                num: self.price_num,
                scale: self.price_scale,
                _pad0: [0u8; 4],
            },
            slot: self.slot,
        }
    }
}

impl Database {
    pub async fn select_oracle_prices_in_slot_range(
        &self,
        oracle_price_history: &Pubkey,
        slot_range: SlotRange,
    ) -> Result<Vec<HistoricalPrice>> {
        let query = match slot_range {
            SlotRange::At(at) => {
                OraclePriceRaw::select_all_at_slot(oracle_price_history.to_string(), at)
            }
            SlotRange::Before(before) => {
                OraclePriceRaw::select_all_before_slot(oracle_price_history.to_string(), before)
            }
            SlotRange::After(after) => {
                OraclePriceRaw::select_all_after_slot(oracle_price_history.to_string(), after)
            }
            SlotRange::Between(start, end) => OraclePriceRaw::select_all_in_slot_range(
                oracle_price_history.to_string(),
                start,
                end,
            ),
        };
        self.fetch_all::<HistoricalPrice, _>(query).await
    }
    pub async fn insert_oracle_prices(
        &self,
        oracle_price_history: &Pubkey,
        prices: &[HistoricalPrice],
    ) -> Result<()> {
        let mut price_nums = vec![];
        let mut price_scales = vec![];
        let mut slots = vec![];
        for p in prices {
            price_nums.push(p.price.num);
            price_scales.push(p.price.scale as i64);
            slots.push(u64_to_i64(p.slot));
        }
        self.execute(OraclePriceRaw::insert_many(
            oracle_price_history.to_string(),
            price_nums,
            price_scales,
            slots,
        ))
        .await?;
        Ok(())
    }

    pub async fn latest_oracle_price(
        &self,
        oracle_price_history: &Pubkey,
    ) -> Result<Option<OraclePrice>> {
        self.fetch_optional(OraclePriceRaw::latest_entry(
            oracle_price_history.to_string(),
        ))
        .await
    }

    pub async fn fetch_latest_entry_by_mint(&self, mint: &Pubkey) -> Result<Option<OraclePrice>> {
        self.fetch_optional(OraclePriceRaw::latest_entry_by_mint(mint.to_string()))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helpers::{connect_to_test_db, random_pubkey};
    use lollys_lotto::{HistoricalDecimal, OracleType};

    #[tokio::test]
    async fn oracle_price_methods() {
        let db = connect_to_test_db().await;
        let info = db
            .insert_oracle_price_history_info(
                &random_pubkey(),
                &random_pubkey(),
                OracleType::Pyth,
                &random_pubkey(),
            )
            .await
            .unwrap();
        let oracle_price_history = info.address;

        let entry = db.latest_oracle_price(&oracle_price_history).await.unwrap();
        assert!(entry.is_none());

        let latest_entry = HistoricalPrice {
            price: HistoricalDecimal {
                num: 1111,
                scale: 4,
                _pad0: [0u8; 4],
            },
            slot: 11001203,
        };
        let inserted_prices = vec![
            HistoricalPrice {
                price: HistoricalDecimal {
                    num: 1000,
                    scale: 4,
                    _pad0: [0u8; 4],
                },
                slot: 1,
            },
            HistoricalPrice {
                price: HistoricalDecimal {
                    num: 1001,
                    scale: 4,
                    _pad0: [0u8; 4],
                },
                slot: 4,
            },
            HistoricalPrice {
                price: HistoricalDecimal {
                    num: 1004,
                    scale: 4,
                    _pad0: [0u8; 4],
                },
                slot: 7,
            },
            HistoricalPrice {
                price: HistoricalDecimal {
                    num: 1010,
                    scale: 4,
                    _pad0: [0u8; 4],
                },
                slot: 11,
            },
            latest_entry.clone(),
        ];
        db.insert_oracle_prices(&oracle_price_history, &inserted_prices)
            .await
            .unwrap();

        let entry = db
            .latest_oracle_price(&oracle_price_history)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(latest_entry.slot, entry.slot);
        assert_eq!(latest_entry.price.num, entry.price_num);
        assert_eq!(latest_entry.price.scale, entry.price_scale);

        let prices = db
            .select_oracle_prices_in_slot_range(&oracle_price_history, SlotRange::After(4))
            .await
            .unwrap();
        assert!(!prices.contains(&inserted_prices[0]));
        assert!(prices.contains(&inserted_prices[1]));
        assert!(prices.contains(&inserted_prices[2]));
        assert!(prices.contains(&inserted_prices[3]));
        assert!(prices.contains(&inserted_prices[4]));

        let prices = db
            .select_oracle_prices_in_slot_range(&oracle_price_history, SlotRange::Before(4))
            .await
            .unwrap();
        assert!(prices.contains(&inserted_prices[0]));
        assert!(prices.contains(&inserted_prices[1]));
        assert!(!prices.contains(&inserted_prices[2]));
        assert!(!prices.contains(&inserted_prices[3]));
        assert!(!prices.contains(&inserted_prices[4]));

        let prices = db
            .select_oracle_prices_in_slot_range(&oracle_price_history, SlotRange::At(7))
            .await
            .unwrap();
        assert!(!prices.contains(&inserted_prices[0]));
        assert!(!prices.contains(&inserted_prices[1]));
        assert!(prices.contains(&inserted_prices[2]));
        assert!(!prices.contains(&inserted_prices[3]));
        assert!(!prices.contains(&inserted_prices[4]));

        let prices = db
            .select_oracle_prices_in_slot_range(&oracle_price_history, SlotRange::At(8))
            .await
            .unwrap();
        assert!(prices.is_empty());

        let prices = db
            .select_oracle_prices_in_slot_range(&oracle_price_history, SlotRange::Between(3, 10))
            .await
            .unwrap();
        assert!(!prices.contains(&inserted_prices[0]));
        assert!(prices.contains(&inserted_prices[1]));
        assert!(prices.contains(&inserted_prices[2]));
        assert!(!prices.contains(&inserted_prices[3]));
        assert!(!prices.contains(&inserted_prices[4]));
    }
}
