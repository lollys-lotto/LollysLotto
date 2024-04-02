use crate::instructions::*;

pub fn create_lolly_burn_state(
    payer: &Pubkey,  
    authority: &Pubkey,
    lolly_burn_state: &Pubkey,
    lolly_mint: &Pubkey,
    lolly_burn_state_lolly_vault: &Pubkey,
    usdc_mint: &Pubkey,
    lolly_burn_state_usdc_vault: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::CreateLollyBurnState.data();

    let accounts = lollys_lotto::accounts::CreateLollyBurnState{
        payer: *payer,
        authority: *authority,
        lolly_burn_state: *lolly_burn_state,
        lolly_mint: *lolly_mint,
        lolly_burn_state_lolly_vault: *lolly_burn_state_lolly_vault,
        usdc_mint: *usdc_mint,
        lolly_burn_state_usdc_vault: *lolly_burn_state_usdc_vault,
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