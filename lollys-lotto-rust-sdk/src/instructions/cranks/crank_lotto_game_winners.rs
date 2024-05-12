use lollys_lotto::state::LottoTicketNumbers;

use crate::instructions::*;
use lollys_lotto::instruction::CrankLottoGameWinners;

pub fn crank_lotto_game_winner(
    round: u64,
    winning_numbers: LottoTicketNumbers,
    winning_numbers_index: [i64; 4],
    authority: &Pubkey,
    user: &Pubkey,
    user_metadata: &Pubkey,
    lotto_game: &Pubkey,
    lotto_game_vault_signer: &Pubkey,
    lotto_game_vault: &Pubkey,
    lotto_ticket: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = CrankLottoGameWinners {
        round,
        winning_numbers,
        winning_numbers_index,
    }
    .data();
    let accounts = lollys_lotto::accounts::CrankLottoGameWinners {
        authority: *authority,
        lotto_game: *lotto_game,
        lotto_game_vault_signer: *lotto_game_vault_signer,
        lotto_game_vault: *lotto_game_vault,
        user: *user,
        user_metadata: *user_metadata,
        lotto_ticket: *lotto_ticket,
        event_emitter: *event_emitter,
        token_program: token::ID,
    }
    .to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}
