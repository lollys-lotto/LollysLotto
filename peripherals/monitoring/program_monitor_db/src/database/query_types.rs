use crate::{utils::type_conversions::u64_to_i64, SSLv2DatabaseError};
use sqlx::types::chrono::NaiveDateTime;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum SlotRange {
    At(i64),
    Before(i64),
    After(i64),
    Between(i64, i64),
}

impl SlotRange {
    pub fn at(slot: u64) -> Self {
        Self::At(u64_to_i64(slot))
    }

    pub fn before(slot: u64) -> Self {
        Self::Before(u64_to_i64(slot))
    }

    pub fn after(slot: u64) -> Self {
        Self::After(u64_to_i64(slot))
    }

    pub fn between(start: u64, end: u64) -> crate::Result<Self> {
        if start >= end {
            return Err(SSLv2DatabaseError::InvalidSlotRange(start, end));
        }
        Ok(Self::Between(u64_to_i64(start), u64_to_i64(end)))
    }
}

impl Display for SlotRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SlotRange::At(s) => {
                write!(f, "at slot {s}")
            }
            SlotRange::Before(s) => {
                write!(f, "before slot {s}")
            }
            SlotRange::After(s) => {
                write!(f, "after slot {s}")
            }
            SlotRange::Between(start, end) => {
                write!(f, "between slots {start} and {end}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateTimeRange {
    At(NaiveDateTime),
    Before(NaiveDateTime),
    After(NaiveDateTime),
    Between(NaiveDateTime, NaiveDateTime),
}

impl DateTimeRange {
    pub fn at(datetime: NaiveDateTime) -> Self {
        Self::At(datetime)
    }

    pub fn before(datetime: NaiveDateTime) -> Self {
        Self::Before(datetime)
    }

    pub fn after(datetime: NaiveDateTime) -> Self {
        Self::After(datetime)
    }

    pub fn between(start: NaiveDateTime, end: NaiveDateTime) -> crate::Result<Self> {
        if start >= end {
            return Err(SSLv2DatabaseError::InvalidDateTimeRange(
                start.to_string(),
                end.to_string(),
            ));
        }
        Ok(Self::Between(start, end))
    }
}

impl Display for DateTimeRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTimeRange::At(dt) => {
                write!(f, "at {dt}")
            }
            DateTimeRange::Before(dt) => {
                write!(f, "before {dt}")
            }
            DateTimeRange::After(dt) => {
                write!(f, "after {dt}")
            }
            DateTimeRange::Between(start, end) => {
                write!(f, "between {start} and {end}")
            }
        }
    }
}
