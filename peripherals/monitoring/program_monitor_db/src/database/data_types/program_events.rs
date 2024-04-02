use crate::{
    database::{Database, TypedQuery},
    error::Result,
    utils::type_conversions::{
        i16_to_u8, i64_to_u64, naive_datetime_serde, option_pubkey, signature, u64_to_i64,
        u8_to_i16,
    },
};
use lollys_lotto::{SSLProgramEvent, SSLProgramEventData};
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use sqlx::{query_as, types::chrono::NaiveDateTime};
use std::str::FromStr;

/// Represents the various kinds of address that we save.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "instruction_type", rename_all = "snake_case")]
pub enum InstructionType {
    Deposit,
    Withdraw,
    ClaimFees,
    CreateLiquidityAccount,
    CloseLiquidityAccount,
    Swap,
    InternalSwap,
    CreatePoolRegistry,
    ConfigPoolRegistry,
    CreateSSL,
    ConfigSSL,
    CreatePair,
    ConfigPair,
    ConfigSuspendAdmin,
    SuspendSsl,
}

impl From<&SSLProgramEvent> for InstructionType {
    fn from(value: &SSLProgramEvent) -> Self {
        match value.data {
            SSLProgramEventData::Deposit(_) => InstructionType::Deposit,
            SSLProgramEventData::Withdraw(_) => InstructionType::Withdraw,
            SSLProgramEventData::ClaimFees(_) => InstructionType::ClaimFees,
            SSLProgramEventData::CreateLiquidityAccount(_) => {
                InstructionType::CreateLiquidityAccount
            }
            SSLProgramEventData::CloseLiquidityAccount(_) => InstructionType::CloseLiquidityAccount,
            SSLProgramEventData::Swap(_) => InstructionType::Swap,
            SSLProgramEventData::InternalSwap(_) => InstructionType::InternalSwap,
        }
    }
}

/// Raw type for DB I/O
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct ProgramEventRowRaw {
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

impl ProgramEventRowRaw {
    fn select_by_signature(signature: &Signature) -> TypedQuery<Self> {
        query_as::<_, Self>(
            r#"SELECT * FROM program_events
            WHERE transaction_signature = $1 AND error_message IS NULL"#,
        )
        .bind(signature.to_string())
    }

    /// This select query joins data from the Transactions table.
    fn select_by_sequence_num(seq_num: i64) -> TypedQuery<Self> {
        query_as::<_, Self>(
            r#"SELECT * FROM program_events
            WHERE seq_num = $1 AND error_message IS NULL"#,
        )
        .bind(seq_num)
    }

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
    ) -> TypedQuery<Self> {
        query_as::<_, Self>(
            r#"INSERT INTO program_events (
                seq_num,
                version,
                instruction_type,
                transaction_signature,
                data,
                slot,
                block_time,
                error_program,
                error_message
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
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
                error_message
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
    }

    fn highest_seq_num() -> TypedQuery<Self> {
        query_as::<_, Self>(
            r#"
        SELECT * FROM program_events
            WHERE seq_num = (
               SELECT MAX(seq_num)
               FROM program_events
            ) AND error_message IS NULL
        "#,
        )
    }

    fn select_events_by_seq_num_range(start_seq_num: i64, end_seq_num: i64) -> TypedQuery<Self> {
        let query = r#"SELECT *
                    FROM program_events
                    WHERE seq_num >= $1 AND seq_num <= $2 AND error_message IS NULL
                    ORDER BY seq_num desc"#;

        query_as::<_, Self>(query)
            .bind(start_seq_num)
            .bind(end_seq_num)
    }
}

impl Into<ProgramEventRow> for ProgramEventRowRaw {
    fn into(self) -> ProgramEventRow {
        ProgramEventRow {
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
pub struct ProgramEventRow {
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
    pub async fn insert_program_event(
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
    ) -> Result<ProgramEventRow> {
        self.fetch_one(ProgramEventRowRaw::insert(
            seq_num,
            u8_to_i16(version),
            instruction_type,
            data,
            transaction_signature.to_string(),
            u64_to_i64(slot),
            block_time,
            error_program.map(|p| p.to_string()),
            error_message,
        ))
        .await
    }

    pub async fn select_event_by_transaction_signature(
        &self,
        transaction_signature: &Signature,
    ) -> Result<ProgramEventRow> {
        self.fetch_one(ProgramEventRowRaw::select_by_signature(
            transaction_signature,
        ))
        .await
    }

    pub async fn select_event_by_seq_num(&self, seq_num: i64) -> Result<Vec<ProgramEventRow>> {
        self.fetch_all(ProgramEventRowRaw::select_by_sequence_num(seq_num))
            .await
    }

    pub async fn highest_seq_num(&self) -> Result<ProgramEventRow> {
        self.fetch_one(ProgramEventRowRaw::highest_seq_num()).await
    }

    pub async fn fetch_events_by_seq_num_range(
        &self,
        start_seq_num: i64,
        end_seq_num: i64,
    ) -> Result<Vec<ProgramEventRow>> {
        self.fetch_all(ProgramEventRowRaw::select_events_by_seq_num_range(
            start_seq_num,
            end_seq_num,
        ))
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helpers::{connect_to_test_db, insert_program_event};

    #[tokio::test]
    async fn program_events_methods() {
        let db = connect_to_test_db().await;
        let seq_num = 10_000;
        let insertion_result = insert_program_event(
            0,
            InstructionType::CreateLiquidityAccount,
            Some(seq_num),
            None,
            None,
        )
        .await;

        let select_result = db
            .select_event_by_transaction_signature(&insertion_result.transaction_signature)
            .await
            .unwrap();
        assert_eq!(insertion_result, select_result);

        let select_result = db.select_event_by_seq_num(seq_num).await.unwrap();
        assert!(select_result.contains(&insertion_result));

        let _insertion_result2 = insert_program_event(
            2,
            InstructionType::CreateLiquidityAccount,
            Some(10_001),
            None,
            None,
        )
        .await;

        let insertion_result3 = insert_program_event(
            2000,
            InstructionType::CreateLiquidityAccount,
            Some(i64::MAX - 2000),
            None,
            None,
        )
        .await;

        let newest_event = db.highest_seq_num().await.unwrap();
        assert_eq!(i64::MAX - 2000, newest_event.seq_num);
        assert_eq!(insertion_result3, newest_event);
    }
}
