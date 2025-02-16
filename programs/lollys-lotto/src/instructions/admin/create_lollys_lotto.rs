pub use anchor_lang::prelude::*;

use crate::{
    pda_identifier::PDAIdentifier,
    state::{CreateLollysLottoEvent, EventEmitter, LollysLotto, LollysLottoProgramEventData},
};

#[derive(Accounts)]
pub struct CreateLollysLotto<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<LollysLotto>(),
        seeds = [
            LollysLotto::IDENT,
            authority.key().as_ref(),
        ],
        bump,
    )]
    pub lollys_lotto: Box<Account<'info, LollysLotto>>,
    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateLollysLotto<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let lollys_lotto = &mut self.lollys_lotto;
        lollys_lotto.authority = *self.authority.key;
        lollys_lotto.lotto_game_count = 0;
        lollys_lotto.bump = bump;

        let block_time = Clock::get()?.unix_timestamp;

        self.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::CreateLollysLotto(CreateLollysLottoEvent {
                authority: *self.authority.key,
                lollys_lotto: *lollys_lotto.to_account_info().key,
                lotto_game_count: lollys_lotto.lotto_game_count,
            }),
        )?;
        Ok(())
    }
}
