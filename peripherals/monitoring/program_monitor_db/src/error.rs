use regex::Regex;
use sqlx::postgres::PgDatabaseError;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, LollysLottoDatabaseError>;

/// Each variant here is used as a foreign key
/// somewhere in the schema.
#[derive(Debug, Clone, PartialEq)]
pub enum ForeignKey {
    ProgramEvent,
    PoolVault,
    FeeVault,
    LiquidityAccount,
    Pair,
    OraclePriceHistory,
    PoolRegistry,
    SSLPool,
    Unknown,
}

impl Display for ForeignKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LollysLottoDatabaseError {
    #[error("Failed to connect to the database: {0}")]
    DatabaseConnectionError(sqlx::Error),
    #[error("Invalid database URL: {0}: {1}")]
    InvalidDatabaseUrl(String, String),
    #[error("Missing foreign key {0}: {1}")]
    MissingForeignKey(ForeignKey, sqlx::Error),
    #[error("Duplicate key value on constraint {0}: {1}")]
    DuplicateKeyValue(String, sqlx::Error),
    #[error("Row not found {0}")]
    RowNotFound(sqlx::Error),
    #[error("SQLx Error: {0}")]
    OtherSqlxError(sqlx::Error),
    #[error("u64 from bytes requires an 8-byte slice, received a slice of length {0}")]
    BytesToU64Error(usize),
    #[error("u128 from bytes requires a 16-byte slice, received a slice of length {0}")]
    BytesToU128Error(usize),
    #[error("Invalid slot range {0} to {1}, start slot must be less than end slot, end slot must be less than i64::MAX")]
    InvalidSlotRange(u64, u64),
    #[error("Invalid NaiveDateTime range {0} to {1}, start must be before end")]
    InvalidDateTimeRange(String, String),
}

impl From<sqlx::Error> for LollysLottoDatabaseError {
    fn from(e: sqlx::Error) -> Self {
        match &e {
            sqlx::Error::Database(db_err) => {
                if let Some(err) = db_err.try_downcast_ref::<PgDatabaseError>() {
                    if err.code() == FOREIGN_KEY_VIOLATION_ERR_CODE {
                        let fk = table_with_missing_row(err.detail().unwrap_or(""));
                        LollysLottoDatabaseError::MissingForeignKey(fk, e)
                    } else if err.code() == DUPLICATE_KEY_VIOLATION_ERR_CODE {
                        LollysLottoDatabaseError::DuplicateKeyValue(
                            err.constraint().unwrap().to_string(),
                            e,
                        )
                    } else {
                        LollysLottoDatabaseError::OtherSqlxError(e)
                    }
                } else {
                    LollysLottoDatabaseError::OtherSqlxError(e)
                }
            }
            _ => LollysLottoDatabaseError::OtherSqlxError(e),
        }
    }
}

const FOREIGN_KEY_VIOLATION_ERR_CODE: &str = "23503";
const DUPLICATE_KEY_VIOLATION_ERR_CODE: &str = "23505";

fn table_with_missing_row(details: &str) -> ForeignKey {
    let re = Regex::new(".*is not present in table \"(.*)\".*").unwrap();
    let captures = re.captures(details);
    let table: Option<&str> = captures.map(|c| c.get(1).map(|m| m.as_str())).flatten();
    match table {
        Some("program_events") => ForeignKey::ProgramEvent,
        Some("pool_vault_info") => ForeignKey::PoolVault,
        Some("fee_vault_info") => ForeignKey::FeeVault,
        Some("liquidity_account_info") => ForeignKey::LiquidityAccount,
        Some("pair_info") => ForeignKey::Pair,
        _ => ForeignKey::Unknown,
    }
}
