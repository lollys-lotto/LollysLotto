use crate::instructions::*;

pub fn close_lolly_burn_state(
    lolly_burn_state: &Pubkey,
    event_emitter: &Pubkey,
    authority: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::CloseLollyBurnState.data();
    let accounts = lollys_lotto::accounts::CloseLollyBurnState {
        authority: *authority,
        lolly_burn_state: *lolly_burn_state,
        event_emitter: *event_emitter,
        system_program: system_program::ID,
    }
    .to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}
