use crate::instructions::*;

pub fn start_lotto_game(
    round: u64,
    ticket_price: u64,
    game_duration: u64,
    round_name: String,
    authority: &Pubkey,
    lollys_lotto: &Pubkey,
    lotto_game: &Pubkey,
    lotto_game_vault_signer: &Pubkey,
    lotto_game_vault: &Pubkey,
    lotto_game_mint: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::StartLottoGame{
        round,
        ticket_price,
        game_duration,
        round_name,
    }.data();

    let accounts = lollys_lotto::accounts::StartLottoGame{
        authority: *authority,
        lollys_lotto: *lollys_lotto,
        lotto_game: *lotto_game,
        lotto_game_vault_signer: *lotto_game_vault_signer,
        lotto_game_vault: *lotto_game_vault,
        lotto_game_mint: *lotto_game_mint,
        event_emitter: *event_emitter,
        token_program: token::ID,
        associated_token_program: associated_token::ID,
        system_program: system_program::ID,
    }.to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}