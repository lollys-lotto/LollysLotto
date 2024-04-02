use crate::{Database, InstructionType, ProgramEventRow};
use lazy_static::lazy_static;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use std::sync::Mutex;

pub const PG_URL: &str = "postgresql://local:test@127.0.0.1:5432/localdb";

lazy_static! {
    static ref NEW_SEQUENCE_NUM: Mutex<i64> = Mutex::new(0i64);
}

pub fn new_sequence_num() -> i64 {
    let mut seq_num = NEW_SEQUENCE_NUM.lock().unwrap();
    let val = *seq_num;
    *seq_num += 1;
    val
}

pub async fn connect_to_test_db() -> Database {
    Database::new(PG_URL, None).await.unwrap()
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn random_pubkey() -> Pubkey {
    Keypair::new().pubkey()
}

pub async fn insert_program_event(
    slot: u64,
    instruction_type: InstructionType,
    seq_num: Option<i64>,
    signature: Option<Signature>,
    datetime: Option<NaiveDateTime>,
) -> ProgramEventRow {
    connect_to_test_db()
        .await
        .insert_program_event(
            seq_num.unwrap_or(new_sequence_num()),
            0,
            instruction_type,
            "foobar".to_string(),
            &signature.unwrap_or(Signature::new_unique()),
            slot,
            datetime.unwrap_or(now()),
            None,
            None,
        )
        .await
        .unwrap()
}
