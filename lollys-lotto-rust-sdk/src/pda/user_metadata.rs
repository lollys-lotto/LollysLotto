use lollys_lotto::state::UserMetadata;
use solana_program::pubkey::Pubkey;

pub fn get_user_metadata_pda(user: Pubkey) -> Pubkey {
    UserMetadata::address(user)
}

pub fn get_user_metadata_pda_and_bump(user: Pubkey) -> (Pubkey, u8) {
    UserMetadata::address_with_bump(user)
}

pub fn get_user_rewards_vault_address(user: Pubkey) -> Pubkey {
    UserMetadata::user_rewards_vault_address(user)
}
