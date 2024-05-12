use crate::instructions::*;

pub fn close_lollys_lotto(
    authority: &Pubkey,
    lollys_lotto: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::CloseLollysLotto.data();
    let accounts = lollys_lotto::accounts::CloseLollysLotto {
        authority: *authority,
        lollys_lotto: *lollys_lotto,
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
