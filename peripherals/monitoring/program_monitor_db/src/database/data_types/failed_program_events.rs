use crate::{
    database::{Database, TypedQuery},
    error::Result,
    utils::type_conversions::{
        i16_to_u8, i64_to_u64, naive_datetime_serde, option_pubkey, signature, u64_to_i64,
        u8_to_i16,
    },
    InstructionType,
};
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use sqlx::{query_as, types::chrono::NaiveDateTime};
use std::str::FromStr;

/// Raw type for DB I/O
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct FailedProgramEventRowRaw {
    pub id: i64,
    pub seq_num: i64,
    pub version: i16,
    pub instruction_type: InstructionType,
    pub transaction_signature: String,
    pub data: String,
    /// The slot presents in Solana as a `u64`, but Postgres doesn't take `u64`.
    /// Instead, we can store an `i64` under the justified assumption that in real life,
    /// we will never encounter a slot larger than `i64::MAX`.
    pub slot: i64,
    pub block_time: NaiveDateTime,
    pub error_program: Option<String>,
    pub error_message: Option<String>,
}

impl FailedProgramEventRowRaw {
    /// Event ID values are hard-capped to `i64::MAX`.
    /// `transaction_id` is a foreign key, and should exist before attempting this insert.
    fn insert(
        seq_num: i64,
        version: i16,
        instruction_type: InstructionType,
        data: String,
        transaction_signature: String,
        slot: i64,
        block_time: NaiveDateTime,
        error_program: Option<String>,
        error_message: Option<String>,
        execution_error: Option<String>,
    ) -> TypedQuery<Self> {
        query_as::<_, Self>(
            r#"INSERT INTO failed_program_events (
                seq_num,
                version,
                instruction_type,
                transaction_signature,
                data,
                slot,
                block_time,
                error_program,
                error_message,
                execution_error
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING
                id,
                seq_num,
                version,
                instruction_type,
                transaction_signature,
                data,
                slot,
                block_time,
                error_program,
                error_message,
                execution_error
            "#,
        )
        .bind(seq_num)
        .bind(version)
        .bind(instruction_type)
        .bind(transaction_signature)
        .bind(data)
        .bind(slot)
        .bind(block_time)
        .bind(error_program)
        .bind(error_message)
        .bind(execution_error)
    }
}

impl Into<FailedProgramEventRow> for FailedProgramEventRowRaw {
    fn into(self) -> FailedProgramEventRow {
        FailedProgramEventRow {
            id: self.id,
            seq_num: self.seq_num,
            version: i16_to_u8(self.version),
            instruction_type: self.instruction_type,
            transaction_signature: Signature::from_str(&self.transaction_signature).unwrap(),
            data: self.data,
            slot: i64_to_u64(self.slot),
            block_time: self.block_time,
            error_program: self.error_program.map(|p| Pubkey::from_str(&p).unwrap()),
            error_message: self.error_message,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailedProgramEventRow {
    pub id: i64,
    pub seq_num: i64,
    pub version: u8,
    pub instruction_type: InstructionType,
    pub data: String,
    #[serde(with = "signature")]
    pub transaction_signature: Signature,
    pub slot: u64,
    #[serde(default)]
    #[serde(with = "naive_datetime_serde")]
    pub block_time: NaiveDateTime,
    #[serde(with = "option_pubkey")]
    pub error_program: Option<Pubkey>,
    pub error_message: Option<String>,
}

impl Database {
    pub async fn insert_failed_program_event(
        &self,
        seq_num: i64,
        version: u8,
        instruction_type: InstructionType,
        data: String,
        transaction_signature: &Signature,
        slot: u64,
        block_time: NaiveDateTime,
        error_program: Option<Pubkey>,
        error_message: Option<String>,
        execution_error: Option<String>,
    ) -> Result<FailedProgramEventRow> {
        self.fetch_one(FailedProgramEventRowRaw::insert(
            seq_num,
            u8_to_i16(version),
            instruction_type,
            data,
            transaction_signature.to_string(),
            u64_to_i64(slot),
            block_time,
            error_program.map(|p| p.to_string()),
            error_message,
            execution_error,
        ))
        .await
    }
}
