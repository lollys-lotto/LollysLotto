pub use anchor_lang::prelude::*;

use crate::{
    pda_identifier::PDAIdentifier,
    state::{CloseLollysLottoEvent, EventEmitter, LollysLotto, LollysLottoProgramEventData},
};

#[derive(Accounts)]
pub struct CloseLollysLotto<'info> {
    /// CHECK: Authority of the LollysLotto instance
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        close = authority,
        seeds = [
            LollysLotto::IDENT,
            authority.key().as_ref(),
        ],
        bump = lollys_lotto.bump,
    )]
    pub lollys_lotto: Box<Account<'info, LollysLotto>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub system_program: Program<'info, System>,
}

impl<'info> CloseLollysLotto<'info> {
    pub fn process(&mut self) -> Result<()> {
        let block_time = Clock::get()?.unix_timestamp;
        self.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::CloseLollysLotto(CloseLollysLottoEvent {
                lollys_lotto: *self.lollys_lotto.to_account_info().key,
            }),
        )?;
        Ok(())
    }
}
