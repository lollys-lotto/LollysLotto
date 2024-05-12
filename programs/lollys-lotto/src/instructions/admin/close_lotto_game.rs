use crate::{
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{
        CloseLottoGameEvent, EventEmitter, LollysLottoProgramEventData, LottoGame, LottoGameState,
        LottoGameVault,
    },
};
pub use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct CloseLottoGame<'info> {
    /// CHECK: Authority of the EventEmitter instance
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        constraint = lotto_game.load()?.state == LottoGameState::Closed @LollysLottoError::GameNotClosed,
        close = authority,
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
        bump = lotto_game.load()?.lotto_game_vault_bump,
    )]
    pub lotto_game_vault_signer: UncheckedAccount<'info>,

    #[account(
        constraint = lotto_game_vault.amount == 0 @LollysLottoError::LottoGameVaultNotEmpty,
        associated_token::mint = lotto_game.load()?.lotto_game_mint,
        associated_token::authority = lotto_game_vault_signer,
    )]
    pub lotto_game_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub system_program: Program<'info, System>,
}

pub fn close_lotto_game(ctx: Context<CloseLottoGame>) -> Result<()> {
    let block_time = Clock::get()?.unix_timestamp;
    ctx.accounts.event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::CloseLottoGame(CloseLottoGameEvent {
            lotto_game: *ctx.accounts.lotto_game.to_account_info().key,
            round: ctx.accounts.lotto_game.load()?.round,
        }),
    )?;

    Ok(())
}
