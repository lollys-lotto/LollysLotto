use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::{
    constants::LOLLY_MINT,
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{
        lolly_burn_state::LollyBurnState, BurnLollyEvent, EventEmitter, LollysLottoProgramEventData,
    },
};

#[derive(Accounts)]
pub struct BurnLolly<'info> {
    /// This is the token mint that we want to burn
    #[account(mut, address = LOLLY_MINT @LollysLottoError::OnlySwapToLOLLYAllowed)]
    pub lolly_mint: Box<Account<'info, Mint>>,
    /// The authority of the LollyBurnState instance
    pub authority: Signer<'info>,
    // lolly_burn_state account is a PDA signer all the swap, burn CPIs. It is the PDA which will receive USDC fees to its USDC ATA
    #[account(
        mut,
        has_one = authority,
        seeds = [
            LollyBurnState::IDENT,
            authority.key().as_ref()
        ],
        bump = lolly_burn_state.bump)]
    pub lolly_burn_state: Box<Account<'info, LollyBurnState>>,
    /// LOLLY token account to burn tokens, owned by LollyBurnState PDA
    #[account(
        mut,
        associated_token::mint = LOLLY_MINT,
        associated_token::authority = lolly_burn_state,
    )]
    pub lolly_burn_state_lolly_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
    pub token_program: Program<'info, Token>,
}

pub fn burn_lolly(ctx: Context<BurnLolly>) -> Result<()> {
    let authority_pubkey = ctx.accounts.authority.key();
    let lolly_burn_state = &mut ctx.accounts.lolly_burn_state;
    let lolly_mint = &ctx.accounts.lolly_mint;
    let lolly_burn_state_lolly_vault = &ctx.accounts.lolly_burn_state_lolly_vault;

    let lolly_vault_balance = ctx.accounts.lolly_burn_state_lolly_vault.amount;

    msg!(
        "Burning {:?} Lolly tokens",
        ctx.accounts.lolly_burn_state_lolly_vault.amount
    );

    let seeds = &[
        LollyBurnState::IDENT,
        authority_pubkey.as_ref(),
        &[lolly_burn_state.bump],
    ];

    let signer_seeds = &[&seeds[..]];

    // Create the CpiContext we need for the request
    token::burn(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(), 
            Burn {
                mint: lolly_mint.to_account_info(),
                from: lolly_burn_state_lolly_vault.to_account_info(),
                authority: lolly_burn_state.to_account_info(),
            }, 
            signer_seeds,
        ),
        lolly_vault_balance,
    )?;

    lolly_burn_state.total_lolly_burnt += lolly_vault_balance;

    msg!(
        "Total Burnt $LOLLY: {}",
        lolly_burn_state.total_lolly_burnt
    );

    let block_time = Clock::get()?.unix_timestamp;
    ctx.accounts.event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::BurnLolly(BurnLollyEvent {
            authority: *ctx.accounts.authority.key,
            lolly_burn_state: *ctx.accounts.lolly_burn_state.to_account_info().key,
            lolly_burnt_amount_now: lolly_vault_balance,
            total_lolly_burnt: ctx.accounts.lolly_burn_state.total_lolly_burnt,
        }),
    )?;

    Ok(())
}
