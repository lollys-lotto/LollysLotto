use anyhow::{anyhow, Result};
use chrono::{Days, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use program_monitor_db::{DateTimeRange, SlotRange};

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

const YESTERDAY: &'static str = "24 hrs ago";
const NOW: &'static str = "now";

/// 24hr window from the current time.
pub fn yesterday() -> NaiveDateTime {
    now().checked_sub_days(Days::new(1)).unwrap()
}

pub fn parse_n_days_ago(dt: &str) -> Option<NaiveDateTime> {
    dt.strip_suffix(" days ago")
        .or(dt.strip_suffix(" day ago"))
        .and_then(|s| s.parse::<u64>().ok())
        .map(n_days_ago)
}

/// Rounds to midnight of a given day.
pub fn n_days_ago(n: u64) -> NaiveDateTime {
    round_to_midnight(now().checked_sub_days(Days::new(n)).unwrap())
}

pub fn round_to_midnight(dt: NaiveDateTime) -> NaiveDateTime {
    NaiveDateTime::new(dt.date(), NaiveTime::from_hms_opt(0, 0, 0).unwrap())
}

pub fn parse_datetime(dt: Option<String>) -> Result<Option<NaiveDateTime>> {
    dt.map(|dt| {
        if &dt == YESTERDAY {
            return Ok(yesterday());
        }
        if let Some(dt) = parse_n_days_ago(&dt) {
            return Ok(dt);
        }
        if let Ok(dt) = NaiveDate::parse_from_str(&dt, "%Y-%m-%d") {
            return Ok(dt.into());
        }
        if let Ok(dt) = NaiveDateTime::parse_from_str(&dt, "%Y-%m-%dT%H:%M:%S") {
            return Ok(dt);
        }
        if &dt == NOW {
            return Ok(now());
        }
        Err(anyhow!("unknown datetime string: {dt}"))
    })
    .transpose()
}

/// If both values are `None`, this fn defaults to a day ago.
pub fn datetime_range(
    since: Option<NaiveDateTime>,
    until: Option<NaiveDateTime>,
) -> Result<DateTimeRange> {
    Ok(match (since, until) {
        (None, None) => DateTimeRange::After(n_days_ago(1)),
        (Some(since), None) => DateTimeRange::After(since),
        (None, Some(until)) => DateTimeRange::Before(until),
        (Some(since), Some(until)) => DateTimeRange::between(since, until)?,
    })
}

pub fn slot_range(since: Option<u64>, until: Option<u64>) -> Result<SlotRange> {
    match (since, until) {
        (None, None) => Err(anyhow!(
            "Must supply a --since or --until arg for slot range"
        )),
        (Some(since), None) => Ok(SlotRange::after(since)),
        (None, Some(until)) => Ok(SlotRange::before(until)),
        (Some(since), Some(until)) => Ok(SlotRange::between(since, until)?),
    }
}
