pub mod mints;
pub mod state;
pub mod traits;
pub mod user_accounts;
pub use crate::{mints::*, user_accounts::*};
use anchor_lang::solana_program::instruction::Instruction;
use solana_devtools_localnet::{GeneratedAccount, LocalnetConfiguration};
use solana_sdk::{compute_budget::ComputeBudgetInstruction, pubkey};

pub fn primary_localnet() -> anyhow::Result<LocalnetConfiguration> {
    let localnet_accounts = vec![
        NotAdmin.to_localnet_account(),
        NotAdminUsdc.to_localnet_account(),
        TestAdmin.to_localnet_account(),
        TestUser1.to_localnet_account(),
        TestUserUsdc1.to_localnet_account(),
        TestUser2.to_localnet_account(),
        TestUserUsdc2.to_localnet_account(),
        TestUsdc.to_localnet_account(),
    ];

    let config = LocalnetConfiguration::new().accounts(localnet_accounts)?;
    Ok(config)
}

pub fn get_compute_ix() -> Instruction {
    Instruction::new_with_borsh(
        pubkey!("ComputeBudget111111111111111111111111111111"),
        &ComputeBudgetInstruction::SetComputeUnitLimit(1_000_000),
        vec![],
    )
}
