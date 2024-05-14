use lollys_lotto::state::LollyBurnState;
use solana_program::pubkey::Pubkey;

pub fn get_lolly_burn_state_pda(authority: Pubkey) -> Pubkey {
    LollyBurnState::address(authority)
}

pub fn get_lolly_burn_state_pda_and_bump(authority: Pubkey) -> (Pubkey, u8) {
    LollyBurnState::address_with_bump(authority)
}

pub fn get_lolly_burn_state_usdc_vault(authority: Pubkey) -> Pubkey {
    LollyBurnState::usdc_vault(authority)
}

pub fn get_lolly_burn_state_lolly_vault(authority: Pubkey) -> Pubkey {
    LollyBurnState::lolly_vault(authority)
}
