use lollys_lotto::{pda_identifier::PDAIdentifier, state::UserMetadata};
use solana_program::pubkey::Pubkey;

pub fn get_user_metadata_pda(user: &Pubkey, lolly_lotto_program_id: &Pubkey) -> Pubkey {
    let (user_metadata_pda, _) = Pubkey::find_program_address(
        &[UserMetadata::IDENT, user.as_ref()],
        lolly_lotto_program_id,
    );
    user_metadata_pda
}

pub fn get_user_metadata_pda_and_bump(
    user: &Pubkey,
    lolly_lotto_program_id: &Pubkey,
) -> (Pubkey, u8) {
    let (user_metadata_pda, user_metadata_bump) = Pubkey::find_program_address(
        &[UserMetadata::IDENT, user.as_ref()],
        lolly_lotto_program_id,
    );
    (user_metadata_pda, user_metadata_bump)
}
