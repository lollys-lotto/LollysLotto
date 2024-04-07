use crate::instructions::*;

pub fn swap_usdc_lolly(
    data: Vec<u8>,
    authority: &Pubkey,
    lollys_burn_state: &Pubkey,
    lolly_burn_state_usdc_vault: &Pubkey,
    lolly_burn_state_lolly_vault: &Pubkey,
    jupiter_program_id: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::SwapUsdcLolly { data }.data();
    let accounts = lollys_lotto::accounts::SwapUsdcLolly {
        authority: *authority,
        lolly_burn_state: *lollys_burn_state,
        lolly_burn_state_usdc_vault: *lolly_burn_state_usdc_vault,
        lolly_burn_state_lolly_vault: *lolly_burn_state_lolly_vault,
        jupiter_program: *jupiter_program_id,
        token_program: token::ID,
        associated_token_program: associated_token::ID,
        system_program: system_program::ID,
    }
    .to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}
