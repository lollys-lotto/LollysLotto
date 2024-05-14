use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;

use crate::{
    constants::{LOLLY_MINT, USDC_MINT_DEVNET},
    pda_identifier::PDAIdentifier,
};

#[account]
pub struct LollyBurnState {
    pub bump: u8,
    pub total_lolly_burnt: u64,
    pub authority: Pubkey,
}

impl PDAIdentifier for LollyBurnState {
    const IDENT: &'static [u8] = b"lolly-burn-state";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl LollyBurnState {
    pub fn signer_address(authority: Pubkey) -> Pubkey {
        Self::get_address(&[authority.as_ref()])
    }
    pub fn address(authority: Pubkey) -> Pubkey {
        Self::get_address(&[authority.as_ref()])
    }

    pub fn address_with_bump(authority: Pubkey) -> (Pubkey, u8) {
        Self::get_address_with_bump(&[authority.as_ref()])
    }

    pub fn usdc_vault(authority: Pubkey) -> Pubkey {
        get_associated_token_address(&Self::signer_address(authority), &USDC_MINT_DEVNET)
    }

    pub fn lolly_vault(authority: Pubkey) -> Pubkey {
        get_associated_token_address(&Self::signer_address(authority), &LOLLY_MINT)
    }
}
