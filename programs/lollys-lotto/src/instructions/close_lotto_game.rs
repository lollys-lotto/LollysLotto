pub use anchor_lang::prelude::*;

use crate::{
    errors::LollyError,
    pda_identifier::PDAIdentifier,
    state::{LottoGame, LottoGameState},
};

#[derive(Accounts)]
pub struct CloseLottoGame<'info> {
    /// CHECK: Authority of the EventEmitter instance
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        constraint = lotto_game.state == LottoGameState::Closed @LollyError::GameNotClosed,
        close = authority,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.bump,
    )]
    pub lotto_game: Box<Account<'info, LottoGame>>,

    pub system_program: Program<'info, System>,
}

impl<'info> CloseLottoGame<'info> {
    pub fn process(&self) -> Result<()> {
        Ok(())
    }
}
