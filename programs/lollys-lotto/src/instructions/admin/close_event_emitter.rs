pub use anchor_lang::prelude::*;

use crate::{
    pda_identifier::PDAIdentifier,
    state::{CloseEventEmitterEvent, EventEmitter, LollysLottoProgramEventData},
};

#[derive(Accounts)]
pub struct CloseEventEmitter<'info> {
    /// CHECK: Authority of the EventEmitter instance
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [
            EventEmitter::IDENT,
        ],
        bump,
    )]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub system_program: Program<'info, System>,
}

impl<'info> CloseEventEmitter<'info> {
    pub fn process(&mut self) -> Result<()> {
        let block_time = Clock::get()?.unix_timestamp;
        let event_emitter_address = self.event_emitter.key();
        self.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::CloseEventEmitter(CloseEventEmitterEvent {
                event_emitter: event_emitter_address,
            }),
        )?;
        Ok(())
    }
}
