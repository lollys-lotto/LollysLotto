pub use anchor_lang::prelude::*;

use crate::{pda_identifier::PDAIdentifier, state::LollysLotto};

#[derive(Accounts)]
pub struct CloseLollyLotto<'info> {
    /// CHECK: Authority of the LollysLotto instance
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [
            LollysLotto::IDENT,
            authority.key().as_ref(),
        ],
        bump,
    )]
    pub lollys_lotto: Box<Account<'info, LollysLotto>>,

    pub system_program: Program<'info, System>,
}

impl<'info> CloseLollyLotto<'info> {
    pub fn process(&mut self) -> Result<()> {
        Ok(())
    }
}