use lollys_lotto::state::{LottoGame, LottoGameVault};
use solana_program::pubkey::Pubkey;

pub fn get_lotto_game_pda(authority: &Pubkey, round: u64) -> Pubkey {
    LottoGame::address(*authority, round)
}

pub fn get_lotto_game_pda_and_bump(authority: &Pubkey, round: u64) -> (Pubkey, u8) {
    LottoGame::address_with_bump(*authority, round)
}

pub fn get_lotto_game_vault_pda(lotto_game: &Pubkey) -> Pubkey {
    LottoGameVault::vault_address(*lotto_game)
}

pub fn get_lotto_game_vault_signer_pda(lotto_game: &Pubkey) -> Pubkey {
    LottoGameVault::signer_address(*lotto_game)
    // LottoGameVault::address(*lotto_game)
}

pub fn get_lotto_game_vault_signer_pda_and_bump(lotto_game: &Pubkey) -> (Pubkey, u8) {
    LottoGameVault::address_with_bump(*lotto_game)
}
