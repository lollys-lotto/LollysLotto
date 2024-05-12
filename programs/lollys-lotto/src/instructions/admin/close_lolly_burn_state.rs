pub use anchor_lang::prelude::*;

use crate::{
    pda_identifier::PDAIdentifier,
    state::{CloseLollyBurnStateEvent, EventEmitter, LollyBurnState, LollysLottoProgramEventData},
};

#[derive(Accounts)]
pub struct CloseLollyBurnState<'info> {
    /// CHECK: Authority of the LollyBurnState instance
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        close = authority,
        seeds = [
            LollyBurnState::IDENT,
            authority.key().as_ref()
        ],
        bump,
    )]
    pub lolly_burn_state: Box<Account<'info, LollyBurnState>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub system_program: Program<'info, System>,
}

impl<'info> CloseLollyBurnState<'info> {
    pub fn process(&mut self) -> Result<()> {
        let block_time = Clock::get()?.unix_timestamp;

        self.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::CloseLollyBurnState(CloseLollyBurnStateEvent {
                lolly_burn_state: *self.lolly_burn_state.to_account_info().key,
            }),
        )?;
        Ok(())
    }
}
