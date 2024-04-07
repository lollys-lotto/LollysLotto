use crate::instructions::*;

pub fn buy_lotto_ticket(
    round: u64,
    numbers: [u8; 6],
    authority: &Pubkey,
    user: &Pubkey,
    user_metadata: &Pubkey,
    user_usdc_token_account: &Pubkey,
    lotto_game_mint: &Pubkey,
    lotto_game: &Pubkey,
    lotto_game_vault: &Pubkey,
    lotto_ticket: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::BuyLottoTicket { round, numbers }.data();

    let accounts = lollys_lotto::accounts::BuyLottoTicket {
        authority: *authority,
        user: *user,
        user_metadata: *user_metadata,
        user_usdc_token_account: *user_usdc_token_account,
        lotto_game_mint: *lotto_game_mint,
        lotto_game: *lotto_game,
        lotto_game_vault: *lotto_game_vault,
        lotto_ticket: *lotto_ticket,
        event_emitter: *event_emitter,
        token_program: token::ID,
        associated_token_program: associated_token::ID,
        system_program: system_program::ID,
    }
    .to_account_metas(None);

    Instruction {
        program_id: lollys_lotto::id(),
        accounts,
        data,
    }
}
