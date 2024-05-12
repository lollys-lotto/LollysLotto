pub use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{
    constants::USDC_MINT_DEVNET,
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{
        CrankTransferWinningAmountToUserRewardsVaultEvent, EventEmitter,
        LollysLottoProgramEventData, LottoGame, LottoGameVault, LottoTicket, LottoTicketNumbers,
        UserMetadata, WinningAmountDisbursedState, WinningNumberUpdateState,
    },
};

#[derive(Accounts)]
#[instruction(round: u64, winning_numbers: LottoTicketNumbers)]
pub struct CrankTransferWinningAmountToUserRewardsVault<'info> {
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

    /// CHECK: User account
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

    #[account(
        mut,
        associated_token::mint = USDC_MINT_DEVNET,
        associated_token::authority = user_metadata,
    )]
    pub user_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        has_one = lotto_game,
        has_one = user,
        constraint = lotto_ticket.round == lotto_game.load()?.round @LollysLottoError::InvalidRound,
        constraint = lotto_ticket.numbers == winning_numbers @LollysLottoError::InvalidWinningTicket,
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
    pub event_emitter: Account<'info, EventEmitter>,

    pub token_program: Program<'info, Token>,
}

pub fn crank_transfer_winning_amount_to_user_rewards_vault(
    ctx: Context<CrankTransferWinningAmountToUserRewardsVault>,
    round: u64,
    winning_numbers: LottoTicketNumbers,
    number_of_tickets_with_duplicate_numbers: u32,
) -> Result<()> {
    // if winning_numbers is not the same as the lotto_game.winning_numbers, then return
    let lotto_game = &mut *ctx.accounts.lotto_game.load_mut()?;
    let lotto_game_vault = &ctx.accounts.lotto_game_vault;
    let lotto_game_vault_signer = &ctx.accounts.lotto_game_vault_signer;
    let user_metadata = &mut ctx.accounts.user_metadata;
    let user_rewards_vault = &mut ctx.accounts.user_rewards_vault;
    let lotto_ticket = &mut ctx.accounts.lotto_ticket;
    let lotto_ticket_pubkey = lotto_ticket.key();

    // check if the lotto_ticket numbers are present in the winning_numbers of the lotto_game
    let (tier, index) = match lotto_game.get_tier_and_index_by_winning_numbers(lotto_ticket.numbers)
    {
        Ok((tier, index)) => (tier, index),
        Err(e) => return Err(e.into()),
    };

    // For every respective tier and the index check
    // 1. if switchboard_random_numbers_updated is WinningNumberUpdateState::Updated
    // 2. if winning_amount_disbursed is WinningAmountDisbursedState::NotDisbursed
    // 3. if the winning_amount is greater than 0
    // 4. if the winning_amount is less than the total amount in the lotto_game_vault

    match tier {
        0 => {
            if index != 0 {
                return Err(LollysLottoError::InvalidWinningNumberIndex.into());
            }
            if lotto_game
                .jackpot_winning_numbers
                .switchboard_random_numbers_updated
                == WinningNumberUpdateState::NotUpdated
            {
                return Err(LollysLottoError::JackpotWinningNumbersNotUpdated.into());
            }
            if lotto_game.jackpot_winning_numbers.winning_amount_disbursed
                == WinningAmountDisbursedState::Disbursed
            {
                return Err(LollysLottoError::JackpotAmountAlreadyDisbursed.into());
            }
            lotto_game.jackpot_winning_numbers.winning_amount_disbursed =
                WinningAmountDisbursedState::Disbursed;
        }
        1 => {
            if index >= LottoGame::MAX_TIER_1_WINNERS_V1 {
                return Err(LollysLottoError::InvalidWinningNumberIndex.into());
            }
            if lotto_game.tier_1_winning_numbers[index].switchboard_random_numbers_updated
                == WinningNumberUpdateState::NotUpdated
            {
                return Err(LollysLottoError::Tier1WinningNumbersNotUpdated.into());
            }
            if lotto_game.tier_1_winning_numbers[index].winning_amount_disbursed
                == WinningAmountDisbursedState::Disbursed
            {
                return Err(LollysLottoError::Tier1AmountAlreadyDisbursed.into());
            }
            lotto_game.tier_1_winning_numbers[index].winning_amount_disbursed =
                WinningAmountDisbursedState::Disbursed;
        }
        2 => {
            if index >= LottoGame::MAX_TIER_2_WINNERS_V1 {
                return Err(LollysLottoError::InvalidWinningNumberIndex.into());
            }
            if lotto_game.tier_2_winning_numbers[index].switchboard_random_numbers_updated
                == WinningNumberUpdateState::NotUpdated
            {
                return Err(LollysLottoError::Tier2WinningNumbersNotUpdated.into());
            }
            if lotto_game.tier_2_winning_numbers[index].winning_amount_disbursed
                == WinningAmountDisbursedState::Disbursed
            {
                return Err(LollysLottoError::Tier2AmountAlreadyDisbursed.into());
            }
            lotto_game.tier_2_winning_numbers[index].winning_amount_disbursed =
                WinningAmountDisbursedState::Disbursed;
        }
        3 => {
            if index >= LottoGame::MAX_TIER_3_WINNERS_V1 {
                return Err(LollysLottoError::InvalidWinningNumberIndex.into());
            }
            if lotto_game.tier_3_winning_numbers[index].switchboard_random_numbers_updated
                == WinningNumberUpdateState::NotUpdated
            {
                return Err(LollysLottoError::Tier3WinningNumbersNotUpdated.into());
            }
            if lotto_game.tier_3_winning_numbers[index].winning_amount_disbursed
                == WinningAmountDisbursedState::Disbursed
            {
                return Err(LollysLottoError::Tier3AmountAlreadyDisbursed.into());
            }
            lotto_game.tier_3_winning_numbers[index].winning_amount_disbursed =
                WinningAmountDisbursedState::Disbursed;
        }
        _ => return Err(LollysLottoError::InvalidWinningTier.into()),
    }

    lotto_ticket.is_duplicated = number_of_tickets_with_duplicate_numbers;
    lotto_ticket.prize = lotto_ticket
        .prize
        .checked_div((number_of_tickets_with_duplicate_numbers + 1) as u64)
        .ok_or(LollysLottoError::MathError)?;
    user_metadata.total_amount_won += lotto_ticket.prize;

    // Transfer the winning amount to the user_rewards_vault
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: lotto_game_vault.to_account_info(),
                to: user_rewards_vault.to_account_info(),
                authority: lotto_game_vault_signer.to_account_info(),
            },
            &[&[
                LottoGameVault::IDENT,
                ctx.accounts.lotto_game.key().as_ref(),
                &[lotto_game.lotto_game_vault_bump],
            ]],
        ),
        lotto_ticket.prize,
    )?;
    

    let block_time = Clock::get()?.unix_timestamp;
    ctx.accounts.event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::CrankTransferWinningAmountToUserRewardsVault(
            CrankTransferWinningAmountToUserRewardsVaultEvent {
                round: lotto_game.round,
                winning_numbers,
                number_of_tickets_with_duplicate_numbers,
                lotto_game: ctx.accounts.lotto_game.key(),
                user: *ctx.accounts.user.key,
                lotto_ticket: lotto_ticket_pubkey,
                winning_amount: lotto_ticket.prize,
            },
        ),
    )?;

    Ok(())
}
