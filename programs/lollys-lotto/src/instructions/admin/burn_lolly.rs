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
    #[account(address = LOLLY_MINT @LollysLottoError::OnlySwapToLOLLYAllowed)]
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
        constraint = lolly_burn_state_lolly_vault.mint == lolly_mint.key() @LollysLottoError::OnlyLOLLYBuringAllowed,
        associated_token::mint = lolly_mint,
        associated_token::authority = lolly_burn_state,
    )]
    pub lolly_burn_state_lolly_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
    pub token_program: Program<'info, Token>,
}

pub fn burn_lolly(ctx: Context<BurnLolly>) -> Result<()> {
    // if ctx.accounts.lolly_burn_state_lolly_vault.mint != LOLLY_MINT {
    //     return err!(LollysLottoError::OnlyLOLLYBuringAllowed);
    // }

    let lolly_vault_balance = ctx.accounts.lolly_burn_state_lolly_vault.amount;

    msg!(
        "Burning {:?} Lolly tokens",
        ctx.accounts.lolly_burn_state_lolly_vault.amount
    );
    let seeds = &[
        LollyBurnState::IDENT,
        ctx.accounts.lolly_burn_state.authority.as_ref(),
        &[ctx.accounts.lolly_burn_state.bump],
    ];

    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Burn {
        mint: ctx.accounts.lolly_mint.to_account_info(),
        from: ctx.accounts.lolly_burn_state_lolly_vault.to_account_info(),
        authority: ctx.accounts.lolly_burn_state.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    // Create the CpiContext we need for the request
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    // Execute anchor's helper function to burn tokens
    token::burn(cpi_ctx, lolly_vault_balance)?;

    ctx.accounts.lolly_burn_state.total_lolly_burnt += lolly_vault_balance;
    msg!(
        "Total Burnt $LOLLY: {}",
        ctx.accounts.lolly_burn_state.total_lolly_burnt
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
