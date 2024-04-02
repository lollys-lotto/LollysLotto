use crate::instructions::*;

pub fn close_lollys_lotto(
    lollys_lotto: &Pubkey,
    authority: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::CloseLollyLotto.data();
    let accounts = lollys_lotto::accounts::CloseLollyLotto {
        authority: *authority,
        lollys_lotto: *lollys_lotto,
        system_program: system_program::ID,
    }.to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}