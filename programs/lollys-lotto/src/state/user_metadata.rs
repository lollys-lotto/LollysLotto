pub use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;

use crate::pda_identifier::PDAIdentifier;

pub const USER_CLAIM_TICKET_CAPACITY: usize = 64;

use crate::constants::{SEVEN_DAYS, USDC_MINT_DEVNET};


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
    pub claim_tickets: [ClaimTicket; USER_CLAIM_TICKET_CAPACITY],
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

    pub fn user_rewards_holding_address(user: &Pubkey) -> Pubkey {
        get_associated_token_address(
            &Self::address_for_user(user),
            &USDC_MINT_DEVNET,
        )
    }

    pub fn from_buffer(buf: &mut &[u8]) -> Result<Self> {
        Self::try_deserialize(buf)
    }

    pub fn address_for_user(user: &Pubkey) -> Pubkey {
        Self::get_address(&[user.as_ref()])
    }

    pub fn get_claimable_indices(&self, now: i64) -> Vec<u8> {
        self.claim_tickets
            .iter()
            .enumerate()
            .filter(|(_, t)| **t != ClaimTicket::default() && t.created_at < now - SEVEN_DAYS)
            .map(|(i, _)| u8::try_from(i).unwrap())
            .collect()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, AnchorSerialize, AnchorDeserialize)]
#[repr(C)]
pub struct ClaimTicket {
    pub claimed_amount: u64,
    pub created_at: i64,
}

