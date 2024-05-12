use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    constants::{LOLLY_MINT, USDC_MINT_DEVNET},
    pda_identifier::PDAIdentifier,
    state::{CreateLollyBurnStateEvent, EventEmitter, LollyBurnState, LollysLottoProgramEventData},
};

#[derive(Accounts)]
pub struct CreateLollyBurnState<'info> {
    /// CHECK: Payer for paying the rent of LollyBurnState state
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Authority of the LollyBurnState instance
    pub authority: Signer<'info>,
    /// LollyBurnState instance to be created
    // lolly_burn_state account is a PDA signer all the swap, burn CPIs. It is the PDA which will receive USDC fees to its USDC ATA
    #[account(
        init,
        payer = payer,
        seeds=[
            LollyBurnState::IDENT,
            authority.key().as_ref()
        ],
        bump,
        space = 8 + std::mem::size_of::<LollyBurnState>()
    )]
    lolly_burn_state: Box<Account<'info, LollyBurnState>>,
    /// Mint address of the LOLLY token
    #[account(address = LOLLY_MINT)]
    lolly_mint: Box<Account<'info, Mint>>,
    /// LOLLY token account to store LOLLY swapped from USDC of lolly_burn_state_usdc_vault using jupiter owned by LollyBurnState PDA
    #[account(
        init,
        payer = payer,
        associated_token::mint = lolly_mint,
        associated_token::authority = lolly_burn_state,
    )]
    pub lolly_burn_state_lolly_vault: Box<Account<'info, TokenAccount>>,
    /// Mint address of the USDC token
    #[account(address = USDC_MINT_DEVNET)]
    usdc_mint: Box<Account<'info, Mint>>,
    /// USDC token account to store USDC sent from LottoGame USDC vault owned by LollyBurnState PDA
    #[account(
        init,
        payer = payer,
        associated_token::mint = usdc_mint,
        associated_token::authority = lolly_burn_state,
    )]
    pub lolly_burn_state_usdc_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateLollyBurnState<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let lolly_burn_state = &mut self.lolly_burn_state;
        lolly_burn_state.bump = bump;
        lolly_burn_state.authority = self.authority.key();

        let block_time = Clock::get()?.unix_timestamp;
        self.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::CreateLollyBurnState(CreateLollyBurnStateEvent {
                authority: *self.authority.key,
                lolly_burn_state: *lolly_burn_state.to_account_info().key,
                lolly_mint: *self.lolly_mint.to_account_info().key,
                lolly_burn_state_lolly_vault: *self
                    .lolly_burn_state_lolly_vault
                    .to_account_info()
                    .key,
                usdc_mint: *self.usdc_mint.to_account_info().key,
                lolly_burn_state_usdc_vault: *self
                    .lolly_burn_state_usdc_vault
                    .to_account_info()
                    .key,
            }),
        )?;
        Ok(())
    }
}
