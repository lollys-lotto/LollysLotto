pub use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{
    constants::USDC_MINT_DEVNET,
    errors::LollyError,
    pda_identifier::PDAIdentifier,
    state::{LottoGame, LottoGameState, LottoGameVault, LottoTicket, UserMetadata},
};

#[derive(Accounts)]
#[instruction(winning_numbers: [u8; 6])]
pub struct CrankLottoGameWinner<'info> {
    /// CHECK: Authority of the LottoGame instance
    pub authority: AccountInfo<'info>,

    #[account(
        mut,
        has_one = authority,
        has_one = lotto_game_vault,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.bump,
    )]
    pub lotto_game: Box<Account<'info, LottoGame>>,

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

    pub token_program: Program<'info, Token>,

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
        constraint = lotto_ticket.round == lotto_game.round @LollyError::InvalidRound,
        constraint = lotto_ticket.numbers[0] == winning_numbers[0] @LollyError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers[1] == winning_numbers[1] @LollyError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers[2] == winning_numbers[2] @LollyError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers[3] == winning_numbers[3] @LollyError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers[4] == winning_numbers[4] @LollyError::InvalidWinningTicket,
        constraint = lotto_ticket.numbers[5] == winning_numbers[5] @LollyError::InvalidWinningTicket,
        seeds = [
            LottoTicket::IDENT,
            lotto_game.key().as_ref(),
            user_metadata.key().as_ref(),
            winning_numbers[0].to_le_bytes().as_ref(),
            winning_numbers[1].to_le_bytes().as_ref(),
            winning_numbers[2].to_le_bytes().as_ref(),
            winning_numbers[3].to_le_bytes().as_ref(),
            winning_numbers[4].to_le_bytes().as_ref(),
            winning_numbers[5].to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub lotto_ticket: Box<Account<'info, LottoTicket>>,
}

impl<'info> CrankLottoGameWinner<'info> {
    pub fn process(&mut self, winning_numbers: [u8; 6], winning_amount: u64) -> Result<()> {
        let user_metadata = &mut self.user_metadata;
        let lotto_game = &mut self.lotto_game;
        let lotto_ticket = &mut self.lotto_ticket;
        let user_rewards_vault = &self.user_rewards_vault;
        let lotto_game_vault = &self.lotto_game_vault;
        let lotto_game_vault_signer = &self.lotto_game_vault_signer;

        msg!("lotto_game.state: {:?}", lotto_game.state);
        msg!("lotto_game.round: {}", lotto_game.round);
        msg!("lotto_game.start_date: {}", lotto_game.start_date);
        msg!("lotto_game.end_date: {}", lotto_game.end_date);

        let clock = Clock::get()?.unix_timestamp;
        msg!("Clock: {}", clock);
        if clock < lotto_game.end_date {
            return Err(LollyError::GameNotFinished.into());
        }
        // CHECK: Check if the user has won the game
        if lotto_ticket.numbers != winning_numbers {
            return Err(LollyError::InvalidWinningTicket.into());
        }

        // CHECK: Check if the LottoGame is open
        if lotto_game.state != LottoGameState::Open {
            return Err(LollyError::LottoGameNotOpen.into());
        }

        // CHECK: Check if the user has already declared the winner
        if lotto_ticket.is_winner != 0 {
            return Err(LollyError::AlreadyDeclaredWinner.into());
        }

        lotto_game.state = LottoGameState::Finished;

        lotto_game.winning_numbers = winning_numbers;
        lotto_game.winning_ticket = lotto_ticket.key();

        lotto_ticket.is_winner = 1;
        lotto_ticket.prize = winning_amount;

        user_metadata.total_amount_won += winning_amount;
        // // PROCESS: Transfer the prize to the user

        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: lotto_game_vault.to_account_info(),
                    to: user_rewards_vault.to_account_info(),
                    authority: lotto_game_vault_signer.to_account_info(),
                },
                &[&[
                    LottoGameVault::IDENT,
                    lotto_game.key().as_ref(),
                    &[lotto_game.lotto_game_vault_bump],
                ]],
            ),
            winning_amount,
        )?;

        // // PROCESS: Transfer the remaining amount to the lotto game vault
        // token_transfer(
        //     user_rewards_vault,
        //     lotto_game_vault,
        //     lotto_ticket.total_user_ticket_count * lotto_game.ticket_price,
        // )?;

        Ok(())
    }
}
