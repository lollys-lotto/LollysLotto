use crate::instructions::*;
use lollys_lotto::{
    instruction::CrankTransferWinningAmountToUserRewardsVault, state::LottoTicketNumbers,
};

pub fn crank_transfer_winning_amount_to_user_rewards_vault(
    round: u64,
    winning_numbers: LottoTicketNumbers,
    number_of_tickets_with_duplicate_numbers: u32,
    authority: &Pubkey,
    user: &Pubkey,
    user_metadata: &Pubkey,
    user_rewards_vault: &Pubkey,
    lotto_game: &Pubkey,
    lotto_game_vault_signer: &Pubkey,
    lotto_game_vault: &Pubkey,
    lotto_ticket: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = CrankTransferWinningAmountToUserRewardsVault {
        round,
        winning_numbers,
        number_of_tickets_with_duplicate_numbers,
    }
    .data();
    let accounts = lollys_lotto::accounts::CrankTransferWinningAmountToUserRewardsVault {
        authority: *authority,
        user: *user,
        user_metadata: *user_metadata,
        user_rewards_vault: *user_rewards_vault,
        lotto_game: *lotto_game,
        lotto_game_vault_signer: *lotto_game_vault_signer,
        lotto_game_vault: *lotto_game_vault,
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
