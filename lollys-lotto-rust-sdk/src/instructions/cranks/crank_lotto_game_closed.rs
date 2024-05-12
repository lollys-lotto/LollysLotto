use lollys_lotto::instruction::CrankLottoGameClosed;

use crate::instructions::*;

pub fn crank_lotto_game_closed(
    round: u64,
    authority: &Pubkey,
    lotto_game: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = CrankLottoGameClosed { round }.data();
    let accounts = lollys_lotto::accounts::CrankLottoGameClosed {
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
