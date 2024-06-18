use anchor_lang::prelude::*;

use crate::{
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{
        CrankLottoGameClosedEvent, EventEmitter, LollysLottoProgramEventData, LottoGame,
        LottoGameState,
    },
};

#[derive(Accounts)]
#[instruction(round: u64)]
pub struct CrankLottoGameClosed<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority,
        constraint = lotto_game.load()?.round == round @LollysLottoError::InvalidRound,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.load()?.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.load()?.bump,
    )]
    pub lotto_game: AccountLoader<'info, LottoGame>,
    #[account(mut)]
    pub event_emitter: Account<'info, EventEmitter>,
}

pub fn crank_lotto_game_closed(ctx: Context<CrankLottoGameClosed>, round: u64) -> Result<()> {
    // if clock is more than lotto_game.end_time, then close the game
    let lotto_game = &mut *ctx.accounts.lotto_game.load_mut()?;
    if lotto_game.state == LottoGameState::Closed {
        return Err(LollysLottoError::GameAlreadyClosed.into());
    }
    let current_time = Clock::get()?.unix_timestamp;
    if current_time > lotto_game.end_date {
        lotto_game.state = LottoGameState::Closed;
        let block_time = Clock::get()?.unix_timestamp;
        ctx.accounts.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::CrankLottoGameClosed(CrankLottoGameClosedEvent {
                lotto_game: ctx.accounts.lotto_game.key(),
                round,
                ticket_price: lotto_game.ticket_price,
                game_duration: (lotto_game.end_date - lotto_game.start_date) as u64,
            }),
        )?;
    }

    Ok(())
}
