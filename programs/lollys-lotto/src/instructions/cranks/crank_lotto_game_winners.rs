use anchor_lang::prelude::*;
use anchor_lang::Result as AnchorResult;
use anchor_spl::token::{Token, TokenAccount};
use num_traits::ToPrimitive;
use rust_decimal::Decimal;

use crate::state::CrankLottoGameWinnersEvent;
use crate::state::EventEmitter;
use crate::state::LollysLottoProgramEventData;
use crate::state::LottoTicketNumbers;
use crate::{
    constants::USDC_MINT_DEVNET,
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{LottoGame, LottoGameState, LottoGameVault, LottoTicket, UserMetadata},
};

#[derive(Accounts)]
#[instruction(round: u64, winning_numbers: [u8; 6])]
pub struct CrankLottoGameWinners<'info> {
    /// CHECK: Authority of the LottoGame instance
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        has_one = lotto_game_vault,
        constraint = lotto_game.load()?.round == round @LollysLottoError::InvalidRound,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.load()?.bump,
    )]
    pub lotto_game: AccountLoader<'info, LottoGame>,

    /// CHECK: Just a PDA signer
    #[account(
        seeds = [
            LottoGameVault::IDENT,
            lotto_game.key().as_ref(),
        ],
        bump,
    )]
    pub lotto_game_vault_signer: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = USDC_MINT_DEVNET,
        associated_token::authority = lotto_game_vault_signer,
    )]
    pub lotto_game_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: User who is cranking the winner, can be user or anyone
    #[account()]
    pub user: AccountInfo<'info>,

    #[account(
        mut,
        has_one = user,
        seeds = [
            UserMetadata::IDENT,
            user.key().as_ref(),
        ],
        bump = user_metadata.bump,
    )]
    pub user_metadata: Box<Account<'info, UserMetadata>>,

    // #[account(
    //     mut,
    //     associated_token::mint = USDC_MINT_DEVNET,
    //     associated_token::authority = user_metadata,
    // )]
    // pub user_rewards_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        has_one = lotto_game,
        has_one = user,
        constraint = lotto_ticket.round == lotto_game.load()?.round @LollysLottoError::InvalidRound,
        constraint = lotto_ticket.numbers.number1 == winning_numbers[0] @LollysLottoError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers.number2 == winning_numbers[1] @LollysLottoError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers.number3 == winning_numbers[2] @LollysLottoError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers.number4 == winning_numbers[3] @LollysLottoError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers.number5 == winning_numbers[4] @LollysLottoError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers.jackpot_number == winning_numbers[5] @LollysLottoError::InvalidWinningTicket,
        seeds = [
            LottoTicket::IDENT,
            lotto_game.key().as_ref(),
            user_metadata.key().as_ref(),
            lotto_ticket.numbers.number1.to_le_bytes().as_ref(),
            lotto_ticket.numbers.number2.to_le_bytes().as_ref(),
            lotto_ticket.numbers.number3.to_le_bytes().as_ref(),
            lotto_ticket.numbers.number4.to_le_bytes().as_ref(),
            lotto_ticket.numbers.number5.to_le_bytes().as_ref(),
            lotto_ticket.numbers.jackpot_number.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub lotto_ticket: Box<Account<'info, LottoTicket>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub token_program: Program<'info, Token>,
}

pub fn crank_lotto_game_winners(
    ctx: Context<CrankLottoGameWinners>,
    round: u64,
    winning_numbers: LottoTicketNumbers,
    winning_numbers_index: [i64; 4],
) -> AnchorResult<()> {
    let lotto_game = &mut *ctx.accounts.lotto_game.load_mut()?;
    let lotto_ticket = &mut ctx.accounts.lotto_ticket;
    let lotto_game_vault = &mut ctx.accounts.lotto_game_vault;
    let event_emitter = &mut ctx.accounts.event_emitter;

    msg!("lotto_game.state: {:?}", lotto_game.state);
    msg!("lotto_game.round: {}", lotto_game.round);
    msg!("lotto_game.start_date: {}", lotto_game.start_date);
    msg!("lotto_game.end_date: {}", lotto_game.end_date);

    // CHECK: Check if the user has already declared the winner
    if lotto_ticket.is_winner != 0 {
        return Err(LollysLottoError::AlreadyDeclaredWinner.into());
    }

    // CHECK: Check if the LottoGame is open
    if lotto_game.state != LottoGameState::Closed {
        return Err(LollysLottoError::LottoGameIsStillOpen.into());
    }

    let clock = Clock::get()?.unix_timestamp;
    msg!("Clock: {}", clock);
    if clock < lotto_game.end_date {
        return Err(LollysLottoError::LottoGameIsStillOpen.into());
    }

    // CHECK: Check if the user has won the game
    if lotto_ticket.numbers != winning_numbers {
        return Err(LollysLottoError::InvalidWinningTicket.into());
    }

    // all the numbers in winning_numbers_index should be -1 except one, get the index of the winning number
    let mut winning_number_index: i64 = -1;
    let mut winning_tier: i64 = -1;

    for (i, &value) in winning_numbers_index.iter().enumerate() {
        if value != -1 {
            winning_number_index = value;
            winning_tier = i as i64; // casting might be necessary based on the type required elsewhere
            break;
        }
    }

    if winning_number_index == -1 {
        //TODO: emit event
        return Err(LollysLottoError::WinningNumberIndexIsNotProvided.into());
    }

    // winning_tier = 0 => jackpot
    // winning_tier = 1 => tier 1
    // winning_tier = 2 => tier 2
    // winning_tier = 3 => tier 3
    // set the winning numbers in the lotto game based on the winning tier and winning number index
    let lotto_game_vault_amount = lotto_game_vault.amount;
    let amount_to_be_disbursed: Decimal = lotto_game.get_amount_to_be_disbursed(
        winning_tier,
        winning_number_index,
        winning_numbers,
        lotto_game_vault_amount,
    )?;

    lotto_ticket.is_winner = 1;
    lotto_ticket.prize = amount_to_be_disbursed.to_u64().unwrap();

    // user_metadata.total_amount_won += lotto_ticket.prize;
    // // PROCESS: Transfer the prize to the user

    // token::transfer(
    //     CpiContext::new_with_signer(
    //         self.token_program.to_account_info(),
    //         Transfer {
    //             from: lotto_game_vault.to_account_info(),
    //             to: user_rewards_vault.to_account_info(),
    //             authority: lotto_game_vault_signer.to_account_info(),
    //         },
    //         &[&[
    //             LottoGameVault::IDENT,
    //             lotto_game.key().as_ref(),
    //             &[lotto_game.lotto_game_vault_bump],
    //         ]],
    //     ),
    //     lotto_ticket.prize,
    // )?;

    // // PROCESS: Transfer the remaining amount to the lotto game vault
    // token_transfer(
    //     user_rewards_vault,
    //     lotto_game_vault,
    //     lotto_ticket.total_user_ticket_count * lotto_game.ticket_price,
    // )?;

    let block_time = Clock::get()?.unix_timestamp;
    event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::CrankLottoGameWinners(CrankLottoGameWinnersEvent {
            round: lotto_game.round,
            winning_numbers,
            winning_numbers_index,
            winning_user: *ctx.accounts.user.key,
            lotto_ticket: ctx.accounts.lotto_ticket.key(),
            lotto_game: ctx.accounts.lotto_game.key(),
        }),
    )?;

    Ok(())
}
