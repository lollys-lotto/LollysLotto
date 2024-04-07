use crate::instructions::*;

pub fn close_lotto_game(lotto_game: &Pubkey, authority: &Pubkey) -> Instruction {
    let data = lollys_lotto::instruction::CloseLottoGame.data();
    let accounts = lollys_lotto::accounts::CloseLottoGame {
        authority: *authority,
        lotto_game: *lotto_game,
        system_program: system_program::ID,
    }
    .to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}
