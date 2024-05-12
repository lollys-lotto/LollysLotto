pub use anchor_lang::prelude::*;

use crate::{
    pda_identifier::PDAIdentifier,
    state::{CloseUserMetadataEvent, EventEmitter, LollysLottoProgramEventData, UserMetadata},
};

#[derive(Accounts)]
pub struct CloseUserMetadata<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [
            UserMetadata::IDENT,
            user.key().as_ref(),
        ],
        bump = user_metadata.bump,
    )]
    pub user_metadata: Box<Account<'info, UserMetadata>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub system_program: Program<'info, System>,
}

impl<'info> CloseUserMetadata<'info> {
    pub fn process(&mut self) -> Result<()> {
        let block_time = Clock::get()?.unix_timestamp;
        self.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::CloseUserMetadata(CloseUserMetadataEvent {
                user_metadata: *self.user.to_account_info().key,
            }),
        )?;
        Ok(())
    }
}
