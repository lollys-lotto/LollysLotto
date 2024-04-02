use crate::instructions::*;

pub fn crank_lotto_game_winner(
    winning_numbers: [u8; 6],
    winning_amount: u64,
    authority: &Pubkey,
    user: &Pubkey,
    user_metadata: &Pubkey,
    user_rewards_vault: &Pubkey,
    lotto_game: &Pubkey,
    lotto_game_vault_signer: &Pubkey,
    lotto_game_vault: &Pubkey,
    lotto_ticket: &Pubkey,
) -> Instruction {
    let data = lollys_lotto::instruction::CrankLottoGameWinner{
        winning_numbers,
        winning_amount,
    }.data();
    let accounts = lollys_lotto::accounts::CrankLottoGameWinner {
        authority: *authority,
        lotto_game: *lotto_game,
        lotto_game_vault_signer: *lotto_game_vault_signer,
        lotto_game_vault: *lotto_game_vault,
        token_program: token::ID,
        user: *user,
        user_metadata: *user_metadata,
        user_rewards_vault: *user_rewards_vault,
        lotto_ticket: *lotto_ticket,
    }.to_account_metas(None);
    Instruction {
        program_id: lollys_lotto::ID,
        accounts,
        data,
    }
}