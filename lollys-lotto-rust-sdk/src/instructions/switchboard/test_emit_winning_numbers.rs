use crate::instructions::*;

pub fn test_emit_winning_numbers(
    result: [u8; 32],
    authority: &Pubkey,
    lotto_game: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::TestEmitWinningNumbers { result }.data();
    let accounts = lollys_lotto::accounts::TestEmitWinningNumbers {
        authority: *authority,
        lotto_game: *lotto_game,
        event_emitter: *event_emitter,
    }
    .to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}
