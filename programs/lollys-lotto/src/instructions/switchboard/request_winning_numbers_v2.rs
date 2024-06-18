use anchor_lang::prelude::*;

use crate::{errors::LollysLottoError, pda_identifier::PDAIdentifier, state::{EventEmitter, LottoGame}};
use switchboard_on_demand::RandomnessAccountData;


#[derive(Accounts)]
pub struct RequestWinningNumbersV2<'info> {
    /// CHECK: Authority of the LottoGame instance
    pub authority: AccountInfo<'info>,

    #[account(
        has_one = authority,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.load()?.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.load()?.bump,
    )]
    pub lotto_game: AccountLoader<'info, LottoGame>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,


    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    /// The Solana System program. Used to allocate space on-chain for the randomness_request account.
    pub system_program: Program<'info, System>,

}


pub fn request_winning_numbers_v2(
    ctx: Context<RequestWinningNumbersV2>,
) -> anchor_lang::prelude::Result<()> {
let clock: Clock = Clock::get()?;
    let randomness_data = RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();

    if randomness_data.seed_slot != clock.slot - 1 {
        msg!("seed_slot: {}", randomness_data.seed_slot);
        msg!("slot: {}", clock.slot);
        return Err(LollysLottoError::OnDemandRandomnessNotResolved.into());
    }

    Ok(())
}