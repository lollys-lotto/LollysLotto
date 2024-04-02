pub use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{constants::USDC_MINT_DEVNET, pda_identifier::PDAIdentifier, state::{ClaimTicket, CreateUserMetadataEvent, EventEmitter, LollysLottoEventData, UserMetadata, UserTier, USER_CLAIM_TICKET_CAPACITY}};

#[derive(Accounts)]
pub struct CreateUserMetadata<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user, 
        space = 8 + std::mem::size_of::<UserMetadata>(),
        seeds = [
            UserMetadata::IDENT,
            user.key().as_ref(),
        ],  
        bump,
    )]
    pub user_metadata: Box<Account<'info, UserMetadata>>,
    /// Mint address of the USDC token
    #[account(address = USDC_MINT_DEVNET)]
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = user,
        associated_token::mint = usdc_mint,
        associated_token::authority = user_metadata,
    )]
    pub user_rewards_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateUserMetadata<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let user_metadata = &mut self.user_metadata;
        user_metadata.bump = bump;
        user_metadata.user = *self.user.key;
        user_metadata.created_timestamp = Clock::get()?.unix_timestamp;
        user_metadata.tier = UserTier::Bronze;
        user_metadata.total_tickets_purchased = 0;
        user_metadata.total_amount_won = 0;
        user_metadata.total_amount_claimed = 0;
        user_metadata.last_claimed_at = 0;
        user_metadata.referral_count = 0;
        user_metadata.referral_revenue = 0;
        user_metadata.claim_tickets = [ClaimTicket::default(); USER_CLAIM_TICKET_CAPACITY];        

        let clock = Clock::get()?;
        let block_time = clock.unix_timestamp;

        self.event_emitter.emit_new_event(
            Some(block_time), 
            LollysLottoEventData::CreateUserMetadata(CreateUserMetadataEvent {
                user: user_metadata.user,
                user_metadata: user_metadata.key(),
                created_timestamp: user_metadata.created_timestamp,
            }),
        )?;
        Ok(())
    }
}