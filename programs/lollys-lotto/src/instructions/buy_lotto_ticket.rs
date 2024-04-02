pub use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, Token, Mint, TokenAccount, Transfer}};

use crate::{constants::USDC_MINT_DEVNET, errors::LollyError, pda_identifier::PDAIdentifier, state::{BuyLottoTicketEvent, EventEmitter, LollysLottoEventData, LottoGame, LottoGameState, LottoGameVault, LottoTicket, UserMetadata}};

#[derive(Accounts)]
#[instruction(round: u64, numbers: [u8; 6])]
pub struct BuyLottoTicket<'info> {
    /// CHECK: Authority of the LottoGame instance
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            UserMetadata::IDENT,
            user.key().as_ref(),
        ],
        bump = user_metadata.bump,
    )]
    pub user_metadata: Box<Account<'info, UserMetadata>>,

    #[account(
        mut,
        constraint = user_usdc_token_account.amount >= lotto_game.ticket_price @LollyError::InsufficientFunds,
        associated_token::mint = lotto_game_mint,
        associated_token::authority = user,
    )]
    pub user_usdc_token_account: Box<Account<'info, TokenAccount>>,

    /// Needed for account initialization
    #[account(
        address = USDC_MINT_DEVNET,
    )]
    pub lotto_game_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        has_one = authority,
        has_one = lotto_game_vault,
        has_one = lotto_game_mint,
        constraint = lotto_game.round == round @LollyError::InvalidRound,
        constraint = lotto_game.state == LottoGameState::Open @LollyError::LottoGameNotOpen,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.bump,
    )]
    pub lotto_game: Box<Account<'info, LottoGame>>,

    #[account(
        mut,
        associated_token::mint = lotto_game_mint,
        associated_token::authority = LottoGameVault::signer_address(lotto_game.key()),
    )]
    pub lotto_game_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init, 
        payer = user, 
        space = 8 + std::mem::size_of::<LottoTicket>(), 
        seeds = [
            LottoTicket::IDENT,
            lotto_game.key().as_ref(),
            user_metadata.key().as_ref(),
            numbers[0].to_le_bytes().as_ref(),
            numbers[1].to_le_bytes().as_ref(),
            numbers[2].to_le_bytes().as_ref(),
            numbers[3].to_le_bytes().as_ref(),
            numbers[4].to_le_bytes().as_ref(),
            numbers[5].to_le_bytes().as_ref(),
        ], 
        bump
    )]
    pub lotto_ticket: Box<Account<'info, LottoTicket>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyLottoTicket<'info> {
    pub fn process(&mut self, round: u64, numbers: [u8; 6]) -> Result<()> {
        let lotto_game = &mut self.lotto_game;
        let lotto_ticket = &mut self.lotto_ticket;
        let user_usdc_token_account = &self.user_usdc_token_account;
        let lotto_game_vault = &self.lotto_game_vault;
        let user_metadata = &mut self.user_metadata;

        // // Check if the LottoGame is open
        // if lotto_game.state != LottoGameState::Open {
        //     return Err(LollyError::LottoGameNotOpen.into());
        // }

        // // Check if the round is correct
        // if lotto_game.round != round {
        //     return Err(LollyError::InvalidRound.into());
        // }

        // Check the balance of the user's USDC token account to see if they have enough to buy a ticket
        // let user_usdc_balance = user_usdc_token_account.amount;
        // if user_usdc_balance < lotto_game.ticket_price {
        //     return Err(LollyError::InsufficientFunds.into());
        // }

        // Check the time at which the ticket is being purchased with end_date of the LottoGame
        let current_time = Clock::get()?.unix_timestamp;
        if current_time > lotto_game.end_date {
            return Err(LollyError::LottoGameEnded.into());
        }

        // Transfer USDC from user to LottoGameVault
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: user_usdc_token_account.to_account_info(),
                    to: lotto_game_vault.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            lotto_game.ticket_price,
        )?;

        lotto_ticket.user = *self.user.key;
        lotto_ticket.ticket_number = lotto_game.tickets_sold;
        lotto_ticket.lotto_game = *lotto_game.to_account_info().key;
        lotto_ticket.round = round;
        lotto_ticket.numbers = numbers;
        lotto_ticket.is_winner = 0;
        lotto_ticket.prize = 0;

        lotto_game.tickets_sold += 1;

        user_metadata.total_tickets_purchased += 1;

        let clock = Clock::get()?;
        let block_time = clock.unix_timestamp;

        self.event_emitter.emit_new_event(
            Some(block_time), 
            LollysLottoEventData::BuyLottoTicket(BuyLottoTicketEvent {
                user: *self.user.key,
                user_metadata: user_metadata.key(),
                user_ticket_count: user_metadata.total_tickets_purchased,
                lotto_ticket: lotto_ticket.key(),
                lotto_game: lotto_game.key(),
                tickets_sold: lotto_game.tickets_sold,
                round,
                ticket_number: lotto_ticket.ticket_number,
                numbers,
            }),
        )?;

        Ok(())
    }
}