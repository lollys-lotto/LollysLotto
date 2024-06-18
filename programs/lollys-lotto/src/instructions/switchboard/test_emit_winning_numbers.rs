use anchor_lang::prelude::*;

use crate::{
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{
        DuplicateWinningNumbersEvent, EventEmitter, LollysLottoProgramEventData, LottoGame,
        LottoGameState, LottoTicketNumbers, ProcessWinningNumbersEvent,
    },
};

#[derive(Accounts)]
pub struct TestEmitWinningNumbers<'info> {
    /// CHECK: Authority of the LottoGame instance
    pub authority: AccountInfo<'info>,

    #[account(
        mut,
        has_one = authority,
        constraint = lotto_game.load()?.state == LottoGameState::Closed @LollysLottoError::LottoGameNotOpen,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.load()?.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.load()?.bump,
    )]
    pub lotto_game: AccountLoader<'info, LottoGame>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
}

pub fn test_emit_winning_numbers(
    ctx: Context<TestEmitWinningNumbers>,
    result: [u8; 32],
) -> Result<()> {
    msg!("Randomness received: {:?}", result);
    let timestamp = Clock::get()?.unix_timestamp;
    msg!("Timestamp: {:?}", timestamp);
    let event_emitter = &mut ctx.accounts.event_emitter;
    let lotto_game = &mut *ctx.accounts.lotto_game.load_mut()?;

    // copy first 6 bytes of revealed_random_value to winning_numbers
    let mut winning_numbers = [0u8; LottoTicketNumbers::MAX_SLOT_IN_TICKET_V1];
    winning_numbers.copy_from_slice(&result[..6]);
    winning_numbers[0] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number1;
    winning_numbers[1] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number2;
    winning_numbers[2] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number3;
    winning_numbers[3] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number4;
    winning_numbers[4] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number5;
    winning_numbers[5] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.jackpot_number;

    // let winning_numbers: [u8; LottoTicketNumbers::MAX_SLOT_IN_TICKET_V1] = if result.len() == 6 {
    //     // Attempt to convert the Vec<u8> into an array [u8; 6]
    //     let mut arr = [0u8; LottoTicketNumbers::MAX_SLOT_IN_TICKET_V1];
    //     arr.copy_from_slice(&result);
    //     arr[0] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number1;
    //     arr[1] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number2;
    //     arr[2] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number3;
    //     arr[3] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number4;
    //     arr[4] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number5;
    //     arr[5] %= LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.jackpot_number;
    //     arr
    // } else {
    //     // Handle the error case where the vector does not have exactly 6 elements
    //     panic!("Result vector does not contain exactly 6 elements.");
    //     // Alternatively, you can return a default value or handle the error in another way.
    // };

    let (is_duplicate, is_updated, tier_indices) =
        lotto_game.update_winning_numbers(winning_numbers)?;

    let block_time = Clock::get()?.unix_timestamp;
    if is_duplicate {
        event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::DuplicateWinningNumbers(DuplicateWinningNumbersEvent {
                lotto_game: ctx.accounts.lotto_game.key(),
                round: lotto_game.round,
                randomness: result.clone(),
                duplicate_numbers: winning_numbers,
                duplicate_number_detected_index: tier_indices,
            }),
        )?;
    }
    if is_updated {
        event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::ProcessWinningNumbers(ProcessWinningNumbersEvent {
                lotto_game: ctx.accounts.lotto_game.key(),
                round: lotto_game.round,
                randomness: result,
                winning_numbers,
                winning_numbers_updated_index: tier_indices,
            }),
        )?;
    }
    Ok(())
}
