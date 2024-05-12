use crate::{
    constants::USDC_MINT_DEVNET,
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{ClaimUserRewardsEvent, EventEmitter, LollysLottoProgramEventData, UserMetadata},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct ClaimUserRewards<'info> {
    #[account()]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = user,
    )]
    pub user_usdc_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        has_one = user,
        seeds = [
            UserMetadata::IDENT,
            user.key().as_ref(),
        ],
        bump = user_metadata.bump,
    )]
    pub user_metadata: Account<'info, UserMetadata>,

    /// Mint address of the USDC token
    #[account(address = USDC_MINT_DEVNET)]
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = user_rewards_vault.amount > 0 @LollysLottoError::NoRewardsToClaimFromVault,
        associated_token::mint = usdc_mint,
        associated_token::authority = user_metadata,
    )]
    pub user_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ClaimUserRewards<'info> {
    pub fn process(&mut self, amount_to_be_claimed: u64) -> Result<()> {
        let user_metadata = &mut self.user_metadata;

        let amount_left_to_claim =
            user_metadata.total_amount_won - user_metadata.total_amount_claimed;
        if amount_left_to_claim == 0 {
            return Err(LollysLottoError::NoRewardsToClaimFromVault.into());
        }

        if amount_to_be_claimed > amount_left_to_claim {
            return Err(LollysLottoError::NotSufficientRewardsInVault.into());
        }

        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.user_rewards_vault.to_account_info(),
                    to: self.user_usdc_token_account.to_account_info(),
                    authority: user_metadata.to_account_info(),
                },
                &[&[
                    UserMetadata::IDENT,
                    self.user.key().as_ref(),
                    &[user_metadata.bump],
                ]],
            ),
            amount_to_be_claimed,
        )?;

        user_metadata.total_amount_claimed += amount_to_be_claimed;
        user_metadata.last_claimed_at = Clock::get()?.unix_timestamp;

        let clock = Clock::get()?;
        let block_time = clock.unix_timestamp;

        self.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::ClaimUserRewards(ClaimUserRewardsEvent {
                user: user_metadata.user,
                user_metadata: user_metadata.key(),
                user_rewards_vault: self.user_rewards_vault.key(),
                amount_to_be_claimed,
                total_amount_claimed: user_metadata.total_amount_claimed,
            }),
        )?;

        Ok(())
    }
}
