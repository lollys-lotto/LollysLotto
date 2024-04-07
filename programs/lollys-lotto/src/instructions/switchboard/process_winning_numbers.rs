pub use anchor_lang::prelude::*;
use solana_randomness_service::SimpleRandomnessV1Account;
use solana_randomness_service::ID as SolanaRandomnessServiceID;

use crate::pda_identifier::PDAIdentifier;
use crate::state::EventEmitter;
use crate::state::LollysLottoProgramEventData;
use crate::state::LottoGame;
use crate::state::ProcessWinningNumbersEvent;

#[derive(Accounts)]
pub struct ProcessWinningNumbers<'info> {
    /// We need to make sure the randomness service signed this requests so it can only be invoked by a PDA and not a user.
    #[account(
        signer,
        seeds = [b"STATE"],
        seeds::program = SolanaRandomnessServiceID,
        bump = randomness_state.bump,
    )]
    pub randomness_state: Box<Account<'info, solana_randomness_service::State>>,

    pub request: Box<Account<'info, SimpleRandomnessV1Account>>,

    /// CHECK: Authority of the LottoGame instance
    pub authority: AccountInfo<'info>,

    #[account(
        has_one = authority,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.bump,
    )]
    pub lotto_game: Box<Account<'info, LottoGame>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
}

pub fn process_winning_numbers(
    ctx: Context<ProcessWinningNumbers>,
    result: Vec<u8>,
) -> anchor_lang::prelude::Result<()> {
    msg!("Randomness received: {:?}", result);
    let timestamp = Clock::get()?.unix_timestamp;
    msg!("Timestamp: {:?}", timestamp);
    let event_emitter = &mut ctx.accounts.event_emitter;
    let lotto_game = &mut ctx.accounts.lotto_game;
    let clock = Clock::get()?;
    let block_time = clock.unix_timestamp;
    event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::ProcessWinningNumbers(ProcessWinningNumbersEvent {
            round: lotto_game.round,
            randomness: result,
        }),
    )?;
    // let lotto_game = &mut _ctx.accounts.lotto_game;

    // let winning_numbers: [u8; 6] = if result.len() == 6 {
    //     // Attempt to convert the Vec<u8> into an array [u8; 6]
    //     let mut arr = [0u8; 6];
    //     arr.copy_from_slice(&result);
    //     arr
    // } else {
    //     // Handle the error case where the vector does not have exactly 6 elements
    //     panic!("Result vector does not contain exactly 6 elements.");
    //     // Alternatively, you can return a default value or handle the error in another way.
    // };
    // lotto_game.winning_numbers = winning_numbers;
    Ok(())
}
