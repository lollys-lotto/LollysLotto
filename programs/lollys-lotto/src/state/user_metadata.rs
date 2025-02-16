pub use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;

use crate::constants::USDC_MINT_DEVNET;
use crate::pda_identifier::PDAIdentifier;

#[account]
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UserMetadata {
    pub bump: u8,
    pub user: Pubkey,
    pub created_timestamp: i64,
    pub tier: UserTier,
    pub total_tickets_purchased: u64,
    pub total_amount_won: u64,
    pub total_amount_claimed: u64,
    pub last_claimed_at: i64,
    pub referral_count: u64,
    pub referral_revenue: u64,
}

impl PDAIdentifier for UserMetadata {
    const IDENT: &'static [u8] = b"user-metadata";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl UserMetadata {
    pub fn address(user: Pubkey) -> Pubkey {
        Self::get_address(&[user.as_ref()])
    }

    pub fn address_with_bump(user: Pubkey) -> (Pubkey, u8) {
        Self::get_address_with_bump(&[user.as_ref()])
    }

    pub fn user_rewards_vault_address(user: Pubkey) -> Pubkey {
        get_associated_token_address(&Self::address(user), &USDC_MINT_DEVNET)
    }

    pub fn from_buffer(buf: &mut &[u8]) -> Result<Self> {
        Self::try_deserialize(buf)
    }
}

#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
pub enum UserTier {
    // None,
    Bronze,
    // Silver,
    // Gold,
    // Platinum,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, AnchorSerialize, AnchorDeserialize)]
// #[repr(C)]
// pub struct ClaimTicket {
//     pub claimed_amount: u64,
//     pub created_at: i64,
// }
