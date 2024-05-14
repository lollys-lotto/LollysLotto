use crate::instructions::*;
use lollys_lotto::instruction::CrankTransferToBuyAndBurnVault;

pub fn crank_transfer_to_buy_and_burn_vault(
    round: u64,
    authority: &Pubkey,
    lotto_game: &Pubkey,
    lotto_game_vault_signer: &Pubkey,
    lotto_game_vault: &Pubkey,
    lolly_burn_state: &Pubkey,
    lolly_burn_state_usdc_vault: &Pubkey,
    event_emitter: &Pubkey,
) -> Instruction {
    let data = CrankTransferToBuyAndBurnVault { round }.data();
    let accounts = lollys_lotto::accounts::CrankTransferToBuyAndBurnVault {
        authority: *authority,
        lotto_game: *lotto_game,
        lotto_game_vault_signer: *lotto_game_vault_signer,
        lotto_game_vault: *lotto_game_vault,
        lolly_burn_state: *lolly_burn_state,
        lolly_burn_state_usdc_vault: *lolly_burn_state_usdc_vault,
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
