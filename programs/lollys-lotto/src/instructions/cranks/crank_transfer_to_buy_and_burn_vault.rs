pub use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{
    constants::USDC_MINT_DEVNET,
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{
        CrankTransferToBuyAndBurnVaultEvent, EventEmitter, LollyBurnState,
        LollysLottoProgramEventData, LottoGame, LottoGameVault,
    },
};

#[derive(Accounts)]
#[instruction(round: u64)]
pub struct CrankTransferToBuyAndBurnVault<'info> {
    /// CHECK: Authority of the LottoGame instance
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        has_one = lotto_game_vault,
        constraint = lotto_game.load()?.round == round @LollysLottoError::InvalidRound,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.load()?.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.load()?.bump,
    )]
    pub lotto_game: AccountLoader<'info, LottoGame>,

    /// CHECK: Just a PDA signer
    #[account(
        seeds = [
            LottoGameVault::IDENT,
            lotto_game.key().as_ref(),
        ],
        bump,
    )]
    pub lotto_game_vault_signer: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = USDC_MINT_DEVNET,
        associated_token::authority = lotto_game_vault_signer,
    )]
    pub lotto_game_vault: Box<Account<'info, TokenAccount>>,

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
        constraint = lotto_game.load()?.lotto_game_mint == USDC_MINT_DEVNET @LollysLottoError::OnlySwapFromUSDCAllowed,
        associated_token::mint = USDC_MINT_DEVNET,
        associated_token::authority = lolly_burn_state,
    )]
    pub lolly_burn_state_usdc_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub token_program: Program<'info, Token>,
}

pub fn crank_transfer_to_buy_and_burn_vault(
    ctx: Context<CrankTransferToBuyAndBurnVault>,
    round: u64,
) -> Result<()> {
    let lotto_game = &mut ctx.accounts.lotto_game.load()?;
    let lotto_game_pubkey = ctx.accounts.lotto_game.key();
    let lotto_game_vault_signer = &ctx.accounts.lotto_game_vault_signer;
    let lolly_burn_state_usdc_vault = &ctx.accounts.lolly_burn_state_usdc_vault;
    let lotto_game_vault = &ctx.accounts.lotto_game_vault;

    // Transfer USDC from LottoGame vault to LollyBurnState USDC vault
    let buy_and_burn_amount = lotto_game.final_buy_and_burn_amount()?;

    let seeds = &[
        LottoGameVault::IDENT,
        lotto_game_pubkey.as_ref(),
        &[lotto_game.lotto_game_vault_bump],
    ];

    let signer_seeds = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: lotto_game_vault.to_account_info(),
                to: lolly_burn_state_usdc_vault.to_account_info(),
                authority: lotto_game_vault_signer.to_account_info(),
            },
            signer_seeds,
        ),
        buy_and_burn_amount,
    )?;

    // Emit event
    let block_time = Clock::get()?.unix_timestamp;
    ctx.accounts.event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::CrankTransferToBuyAndBurnVault(
            CrankTransferToBuyAndBurnVaultEvent {
                round,
                lotto_game: ctx.accounts.lotto_game.key(),
                lolly_burn_state: ctx.accounts.lolly_burn_state.key(),
                lolly_burn_state_usdc_vault: ctx.accounts.lolly_burn_state_usdc_vault.key(),
                lotto_game_vault: ctx.accounts.lotto_game_vault.key(),
                buy_and_burn_amount,
            },
        ),
    )?;

    Ok(())
}
