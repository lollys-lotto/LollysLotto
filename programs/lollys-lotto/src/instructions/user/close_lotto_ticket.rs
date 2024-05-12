pub use anchor_lang::prelude::*;

use crate::{
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{
        CloseLottoTicketEvent, EventEmitter, LollysLottoProgramEventData, LottoGame,
        LottoGameState, LottoTicket, UserMetadata,
    },
};

#[derive(Accounts)]
#[instruction(round: u64, numbers: [u8; 6])]
pub struct CloseLottoTicket<'info> {
    /// CHECK: Authority of the LottoTicket instance
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            UserMetadata::IDENT,
            user.key().as_ref(),
        ],
        bump = user_metadata.bump,
    )]
    pub user_metadata: Box<Account<'info, UserMetadata>>,

    #[account(
        constraint = lotto_game.load()?.state == LottoGameState::Closed @LollysLottoError::LottoGameIsStillOpen,
        constraint = lotto_game.load()?.round == round @LollysLottoError::InvalidRound,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.load()?.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.load()?.bump,
    )]
    pub lotto_game: AccountLoader<'info, LottoGame>,
    #[account(
        mut,
        close = user,
        constraint = lotto_ticket.numbers.number1 == numbers[0] @LollysLottoError::InvalidNumbersInTicket,
        constraint = lotto_ticket.numbers.number2 == numbers[1] @LollysLottoError::InvalidNumbersInTicket,
        constraint = lotto_ticket.numbers.number3 == numbers[2] @LollysLottoError::InvalidNumbersInTicket,
        constraint = lotto_ticket.numbers.number4 == numbers[3] @LollysLottoError::InvalidNumbersInTicket,
        constraint = lotto_ticket.numbers.number5 == numbers[4] @LollysLottoError::InvalidNumbersInTicket,
        constraint = lotto_ticket.numbers.jackpot_number == numbers[5] @LollysLottoError::InvalidNumbersInTicket,
        seeds = [
            LottoTicket::IDENT,
            lotto_game.key().as_ref(),
            user_metadata.key().as_ref(),
            numbers[0].to_le_bytes().as_ref(),
            numbers[1].to_le_bytes().as_ref(),
            numbers[2].to_le_bytes().as_ref(),
            numbers[3].to_le_bytes().as_ref(),
            numbers[4].to_le_bytes().as_ref(),
            numbers[5].to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub lotto_ticket: Box<Account<'info, LottoTicket>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub system_program: Program<'info, System>,
}

impl<'info> CloseLottoTicket<'info> {
    pub fn process(&mut self, round: u64, numbers: [u8; 6]) -> Result<()> {
        let block_time = Clock::get()?.unix_timestamp;
        self.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::CloseLottoTicket(CloseLottoTicketEvent {
                round,
                numbers,
                lotto_game: *self.lotto_game.to_account_info().key,
                lotto_ticket: *self.lotto_ticket.to_account_info().key,
                user: *self.user.to_account_info().key,
            }),
        )?;
        Ok(())
    }
}
