use lollys_lotto::{pda_identifier::PDAIdentifier, state::LollysLotto};
use solana_program::pubkey::Pubkey;

pub fn get_lollys_lotto_pda(authority: &Pubkey, lolly_lotto_program_id: &Pubkey) -> Pubkey {
    let (lollys_lotto_pda, _) = Pubkey::find_program_address(
        &[LollysLotto::IDENT, authority.as_ref()],
        lolly_lotto_program_id,
    );
    lollys_lotto_pda
}

pub fn get_lollys_lotto_pda_and_bump(
    authority: &Pubkey,
    lolly_lotto_program_id: &Pubkey,
) -> (Pubkey, u8) {
    let (lollys_lotto_pda, lollys_lotto_bump) = Pubkey::find_program_address(
        &[LollysLotto::IDENT, authority.as_ref()],
        lolly_lotto_program_id,
    );
    (lollys_lotto_pda, lollys_lotto_bump)
}
