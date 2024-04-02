pub use anchor_lang::prelude::*;

use crate::{pda_identifier::PDAIdentifier, state::EventEmitter};

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

impl <'info> CloseEventEmitter<'info> {
    pub fn process(&self) -> Result<()> {
        Ok(())
    }
}
