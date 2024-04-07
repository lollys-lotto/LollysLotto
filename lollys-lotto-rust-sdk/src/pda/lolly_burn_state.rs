use lollys_lotto::{pda_identifier::PDAIdentifier, state::LollyBurnState};
use solana_program::pubkey::Pubkey;

pub fn get_lolly_burn_state_pda(authority: &Pubkey, lolly_lotto_program_id: &Pubkey) -> Pubkey {
    let (lolly_burn_state_pda, _) = Pubkey::find_program_address(
        &[LollyBurnState::IDENT, authority.as_ref()],
        lolly_lotto_program_id,
    );
    lolly_burn_state_pda
}

pub fn get_lolly_burn_state_pda_and_bump(
    authority: &Pubkey,
    lolly_lotto_program_id: &Pubkey,
) -> (Pubkey, u8) {
    let (lolly_burn_state_pda, lolly_burn_state_bump) = Pubkey::find_program_address(
        &[LollyBurnState::IDENT, authority.as_ref()],
        lolly_lotto_program_id,
    );
    (lolly_burn_state_pda, lolly_burn_state_bump)
}
