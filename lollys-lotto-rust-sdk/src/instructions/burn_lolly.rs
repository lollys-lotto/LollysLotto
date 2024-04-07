use crate::instructions::*;

pub fn burn_lolly(
    lolly_mint: &Pubkey,
    authority: &Pubkey,
    lolly_burn_state: &Pubkey,
    lolly_burn_state_lolly_vault: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::BurnLolly.data();

    let accounts = lollys_lotto::accounts::BurnLolly {
        lolly_mint: *lolly_mint,
        authority: *authority,
        lolly_burn_state: *lolly_burn_state,
        lolly_burn_state_lolly_vault: *lolly_burn_state_lolly_vault,
        token_program: token::ID,
    }
    .to_account_metas(None);

    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}
