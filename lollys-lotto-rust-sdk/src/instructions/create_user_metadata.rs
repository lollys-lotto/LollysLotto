use crate::instructions::*;

pub fn create_user_metadata(
    user: &Pubkey,
    user_metadata: &Pubkey,
    usdc_mint: &Pubkey,
    user_rewards_vault: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::CreateUserMetadata.data();

    let accounts = lollys_lotto::accounts::CreateUserMetadata{
        user: *user,
        user_metadata: *user_metadata,
        usdc_mint: *usdc_mint,
        user_rewards_vault: *user_rewards_vault,
        event_emitter: *event_emitter,
        token_program: token::ID,
        associated_token_program: associated_token::ID,
        system_program: system_program::ID,
    }.to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}