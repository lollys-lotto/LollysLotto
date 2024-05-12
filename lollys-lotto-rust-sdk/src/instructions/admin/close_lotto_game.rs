use crate::instructions::*;

pub fn close_lotto_game(
    authority: &Pubkey,
    lotto_game: &Pubkey,
    lotto_game_vault_signer: &Pubkey,
    lotto_game_vault: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::CloseLottoGame.data();
    let accounts = lollys_lotto::accounts::CloseLottoGame {
        authority: *authority,
        lotto_game: *lotto_game,
        lotto_game_vault_signer: *lotto_game_vault_signer,
        lotto_game_vault: *lotto_game_vault,
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
