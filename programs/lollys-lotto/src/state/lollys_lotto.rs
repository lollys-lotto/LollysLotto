use crate::pda_identifier::PDAIdentifier;
use anchor_lang::prelude::*;

#[derive(Debug)]
#[account]
pub struct LollysLotto {
    pub bump: u8,              // 1
    pub lotto_game_count: u64, // 8
    pub authority: Pubkey,     // 32
}

impl PDAIdentifier for LollysLotto {
    const IDENT: &'static [u8] = b"lollys-lotto";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl LollysLotto {
    pub const SIZE: usize = 1 + 8 + 32;

    pub fn signer_address(authority: Pubkey) -> Pubkey {
        Self::get_address(&[authority.as_ref()])
    }
    pub fn address(authority: Pubkey) -> Pubkey {
        Self::get_address(&[authority.as_ref()])
    }

    pub fn address_with_bump(authority: Pubkey) -> (Pubkey, u8) {
        Self::get_address_with_bump(&[authority.as_ref()])
    }
}
