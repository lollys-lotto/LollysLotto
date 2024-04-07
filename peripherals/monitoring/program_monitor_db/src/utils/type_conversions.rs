use crate::{LollysLottoDatabaseError, Result};
use num_traits::cast::ToPrimitive;
use sqlx::types::{chrono::NaiveDateTime, Decimal};

// TODO Maybe just convert some of these to `Result` types,
//   instead of clamping values to a range.

/// Slots are `u64`, stored as `i64` in Postgres.
/// Negative numbers clamped to zero.
/// In practice, this should never occur.
pub fn i64_to_u64(val: i64) -> u64 {
    u64::try_from(val).unwrap_or(0)
}

/// Output value is clamped at `i64::MAX`.
/// In practice, this should never occur.
pub fn u64_to_i64(val: u64) -> i64 {
    i64::try_from(val).unwrap_or(i64::MAX)
}

/// Event versions are presented as `u8`, saved as `i16` in Postgres.
/// Negative numbers clamped to zero.
/// In practice, this should never occur.
pub fn i16_to_u8(val: i16) -> u8 {
    u8::try_from(val).unwrap_or(0)
}

pub fn u8_to_i16(val: u8) -> i16 {
    val as i16
}

/// We always encode using little-endian
pub fn u64_to_byte_array(val: u64) -> Vec<u8> {
    val.to_le_bytes().to_vec()
}

/// Decodes to a little-endian `u64`.
pub fn byte_array_to_u64(val: &[u8]) -> Result<u64> {
    if val.len() != 8 {
        return Err(LollysLottoDatabaseError::BytesToU64Error(val.len()));
    }
    Ok(u64::from_le_bytes(val[0..8].try_into().unwrap()))
}

pub fn u64_to_decimal(val: u64) -> Decimal {
    Decimal::from(val)
}

pub fn decimal_to_u64(val: Decimal) -> u64 {
    val.to_u64().unwrap_or(u64::MAX)
}

/// We always encode using little-endian
pub fn u128_to_byte_array(val: u128) -> Vec<u8> {
    val.to_le_bytes().to_vec()
}

/// Decodes to a little-endian `u128`.
pub fn byte_array_to_u128(val: &[u8]) -> Result<u128> {
    if val.len() != 16 {
        return Err(LollysLottoDatabaseError::BytesToU128Error(val.len()));
    }
    Ok(u128::from_le_bytes(val[0..16].try_into().unwrap()))
}

pub fn u128_to_decimal(val: u128) -> Decimal {
    Decimal::from(val)
}

/// When inserting to DB, some fields are stored twice, once as an array and once as a `Decimal`.
/// The array provides lossless encoding to `u64`,
/// and the `Decimal` allows for math inside SQL queries.
pub fn u64_to_postgres_types(val: u64) -> (Decimal, Vec<u8>) {
    (u64_to_decimal(val), u64_to_byte_array(val))
}

/// When inserting to DB, some fields are stored twice, once as an array and once as a `Decimal`.
/// The array provides lossless encoding to `u128`,
/// and the `Decimal` allows for math inside SQL queries.
pub fn u128_to_postgres_types(val: u128) -> (Decimal, Vec<u8>) {
    (u128_to_decimal(val), u128_to_byte_array(val))
}

pub fn i64_to_naive_datetime(val: i64) -> NaiveDateTime {
    NaiveDateTime::from_timestamp_opt(val, 0).unwrap()
}

pub mod pubkey {
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;

    pub fn serialize<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", pubkey);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Pubkey::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub mod option_pubkey {
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;

    pub fn serialize<S>(pubkey: &Option<Pubkey>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(pubkey) = pubkey {
            let s = format!("{}", pubkey);
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Pubkey>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        if let Some(s) = s {
            Ok(Some(
                Pubkey::from_str(&s).map_err(serde::de::Error::custom)?,
            ))
        } else {
            Ok(None)
        }
    }
}

pub mod signature {
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub use solana_sdk::signature::Signature;
    use std::str::FromStr;

    pub fn serialize<S>(signature: &Signature, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", signature);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Signature, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Signature::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub mod option_signature {
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub use solana_sdk::signature::Signature;
    use std::str::FromStr;

    pub fn serialize<S>(signature: &Option<Signature>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(s) = signature {
            let s = format!("{}", s);
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Signature>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::<String>::deserialize(deserializer)?;
        if let Some(s) = s {
            Ok(Some(
                Signature::from_str(&s).map_err(serde::de::Error::custom)?,
            ))
        } else {
            Ok(None)
        }
    }
}

pub mod option_naive_datetime_serde {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    use sqlx::types::chrono::{DateTime, NaiveDateTime, Utc};

    pub fn serialize<S: Serializer>(
        time: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(time) = time {
            let s = DateTime::<Utc>::from_naive_utc_and_offset(time.clone(), Utc).to_rfc3339();
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<NaiveDateTime>, D::Error> {
        let time: Option<String> = Deserialize::deserialize(deserializer)?;
        if let Some(time) = time {
            Ok(Some(
                DateTime::parse_from_rfc3339(&time)
                    .map_err(D::Error::custom)?
                    .naive_utc(),
            ))
        } else {
            Ok(None)
        }
    }
}

pub mod naive_datetime_serde {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    use sqlx::types::chrono::{DateTime, NaiveDateTime, Utc};

    pub fn serialize<S: Serializer>(
        time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let s = DateTime::<Utc>::from_naive_utc_and_offset(time.clone(), Utc).to_rfc3339();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(DateTime::parse_from_rfc3339(&time)
            .map_err(D::Error::custom)?
            .naive_utc())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use solana_sdk::pubkey::Pubkey;
    use sqlx::types::chrono::NaiveDateTime;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        #[serde(with = "naive_datetime_serde")]
        pub val1: NaiveDateTime,
        #[serde(with = "option_naive_datetime_serde")]
        pub val2: Option<NaiveDateTime>,
        #[serde(with = "option_pubkey")]
        pub val3: Option<Pubkey>,
        #[serde(with = "option_pubkey")]
        pub val4: Option<Pubkey>,
    }

    #[test]
    fn serde_types() {
        let test_struct = TestStruct {
            val1: Default::default(),
            val2: Some(Default::default()),
            val3: Some(Pubkey::new_unique()),
            val4: None,
        };
        let json = serde_json::to_string(&test_struct).unwrap();
        let deser: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(test_struct, deser);
    }
}
