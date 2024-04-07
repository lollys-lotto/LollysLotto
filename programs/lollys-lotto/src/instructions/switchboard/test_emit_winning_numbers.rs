use anchor_lang::prelude::*;

use crate::{
    errors::LollyError,
    pda_identifier::PDAIdentifier,
    state::{
        EventEmitter, LollysLottoProgramEventData, LottoGame, LottoGameState,
        ProcessWinningNumbersEvent,
    },
};

#[derive(Accounts)]
pub struct TestEmitWinningNumbers<'info> {
    /// CHECK: Authority of the LottoGame instance
    pub authority: AccountInfo<'info>,

    #[account(
        mut,
        has_one = authority,
        constraint = lotto_game.state == LottoGameState::Open @LollyError::LottoGameNotOpen,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.bump,
    )]
    pub lotto_game: Box<Account<'info, LottoGame>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
}

pub fn test_emit_winning_numbers(
    ctx: Context<TestEmitWinningNumbers>,
    result: Vec<u8>,
) -> Result<()> {
    msg!("Randomness received: {:?}", result);
    let timestamp = Clock::get()?.unix_timestamp;
    msg!("Timestamp: {:?}", timestamp);
    let event_emitter = &mut ctx.accounts.event_emitter;
    let lotto_game = &mut ctx.accounts.lotto_game;
    let clock = Clock::get()?;
    let block_time = clock.unix_timestamp;
    event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::ProcessWinningNumbers(ProcessWinningNumbersEvent {
            round: lotto_game.round,
            randomness: result,
        }),
    )?;
    Ok(())
}
