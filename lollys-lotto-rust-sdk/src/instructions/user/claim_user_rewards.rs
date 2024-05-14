use crate::instructions::*;

pub fn claim_user_rewards(
    amount_to_be_claimed: u64,
    user: &Pubkey,
    user_usdc_token_account: &Pubkey,
    user_metadata: &Pubkey,
    usdc_mint: &Pubkey,
    user_rewards_vault: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::ClaimUserRewards {
        amount_to_be_claimed,
    }
    .data();

    let accounts = lollys_lotto::accounts::ClaimUserRewards {
        user: *user,
        user_usdc_token_account: *user_usdc_token_account,
        user_metadata: *user_metadata,
        usdc_mint: *usdc_mint,
        user_rewards_vault: *user_rewards_vault,
        event_emitter: *event_emitter,
        token_program: token::ID,
    }
    .to_account_metas(None);

    Instruction {
        program_id: lollys_lotto::id(),
        accounts,
        data,
    }
}