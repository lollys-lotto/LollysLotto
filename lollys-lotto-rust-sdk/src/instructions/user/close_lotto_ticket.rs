use lollys_lotto::state::LottoTicketNumbers;

use crate::instructions::*;

pub fn close_lotto_ticket(
    round: u64,
    numbers: LottoTicketNumbers,
    authority: &Pubkey,
    user: &Pubkey,
    user_metadata: &Pubkey,
    lotto_game: &Pubkey,
    lotto_ticket: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::CloseLottoTicket {
        round,
        numbers,
    }.data();

    let accounts = lollys_lotto::accounts::CloseLottoTicket {
        authority: *authority,
        user: *user,
        user_metadata: *user_metadata,
        lotto_game: *lotto_game,
        lotto_ticket: *lotto_ticket,
        event_emitter: *event_emitter,
        system_program: system_program::ID,
    }
    .to_account_metas(None);

    Instruction {
        program_id: lollys_lotto::id(),
        accounts,
        data,
    }
}