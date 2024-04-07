use lollys_lotto::pda_identifier::PDAIdentifier;
use lollys_lotto::state::{LottoGame, LottoGameVault};
use solana_program::pubkey::Pubkey;

pub fn get_lotto_game_pda(
    authority: &Pubkey,
    round: u64,
    lolly_lotto_program_id: &Pubkey,
) -> Pubkey {
    let (lotto_game_pda, _) = Pubkey::find_program_address(
        &[
            LottoGame::IDENT,
            authority.as_ref(),
            &round.to_le_bytes().as_ref(),
        ],
        lolly_lotto_program_id,
    );
    lotto_game_pda
}

pub fn get_lotto_game_pda_and_bump(
    authority: &Pubkey,
    round: u64,
    lolly_lotto_program_id: &Pubkey,
) -> (Pubkey, u8) {
    let (lotto_game_pda, lotto_game_bump) = Pubkey::find_program_address(
        &[
            LottoGame::IDENT,
            authority.as_ref(),
            &round.to_le_bytes().as_ref(),
        ],
        lolly_lotto_program_id,
    );
    (lotto_game_pda, lotto_game_bump)
}

pub fn get_lotto_game_vault_pda(lotto_game: &Pubkey, lolly_lotto_program_id: &Pubkey) -> Pubkey {
    let (lotto_game_vault_pda, _) = Pubkey::find_program_address(
        &[LottoGameVault::IDENT, lotto_game.as_ref()],
        lolly_lotto_program_id,
    );
    lotto_game_vault_pda
}

pub fn get_lotto_game_vault_pda_and_bump(
    lotto_game: &Pubkey,
    lolly_lotto_program_id: &Pubkey,
) -> (Pubkey, u8) {
    let (lotto_game_vault_pda, lotto_game_vault_bump) = Pubkey::find_program_address(
        &[LottoGameVault::IDENT, lotto_game.as_ref()],
        lolly_lotto_program_id,
    );
    (lotto_game_vault_pda, lotto_game_vault_bump)
}
