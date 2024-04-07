use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::USDC_MINT_DEVNET;
use crate::errors::LollyError;
use crate::pda_identifier::PDAIdentifier;
use crate::state::{
    EventEmitter, LollysLotto, LollysLottoProgramEventData, LottoGame, LottoGameState,
    LottoGameVault, StartLottoGameEvent,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct StartLottoGameParams {
    pub round: u64,
    pub ticket_price: u64,
    pub game_duration: u64,
    pub round_name: String,
}

#[derive(Accounts)]
#[instruction(round: u64)]
pub struct StartLottoGame<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            LollysLotto::IDENT,
            authority.key().as_ref(),
        ],
        bump = lollys_lotto.bump,
    )]
    pub lollys_lotto: Box<Account<'info, LollysLotto>>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<LottoGame>(),
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            round.to_le_bytes().as_ref(),
        ],
        bump,
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

    /// This instruction initializes the token account
    /// required for storing this LottoGame's ticket amount collections in USDC.
    #[account(
        init,
        payer = authority,
        associated_token::mint = lotto_game_mint,
        associated_token::authority = lotto_game_vault_signer,
    )]
    pub lotto_game_vault: Box<Account<'info, TokenAccount>>,

    /// Needed for account initialization
    #[account(
        address = USDC_MINT_DEVNET,
    )]
    pub lotto_game_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> StartLottoGame<'info> {
    pub fn process(
        &mut self,
        bump: u8,
        lotto_game_vault_bump: u8,
        round: u64,
        ticket_price: u64,
        game_duration: u64,
        round_name: String,
    ) -> Result<()> {
        let lotto_game = &mut self.lotto_game;
        let lollys_lotto = &mut self.lollys_lotto;

        if round != lollys_lotto.lotto_game_count {
            return Err(LollyError::RoundNumbersAreSequential.into());
        }

        lotto_game.bump = bump;
        lotto_game.lotto_game_vault_bump = lotto_game_vault_bump;
        lotto_game.authority = *self.authority.key;
        lotto_game.round = lollys_lotto.lotto_game_count;
        lotto_game.start_date = Clock::get()?.unix_timestamp;
        lotto_game.end_date = lotto_game.start_date + game_duration as i64;
        lotto_game.ticket_price = ticket_price;
        lotto_game.tickets_sold = 0;
        lotto_game.lotto_game_mint = *self.lotto_game_mint.to_account_info().key;
        lotto_game.lotto_game_vault = *self.lotto_game_vault.to_account_info().key;
        lotto_game.winning_numbers = [0; 6];
        lotto_game.winning_ticket = Pubkey::default();
        lotto_game.state = LottoGameState::Open;

        lollys_lotto.lotto_game_count += 1;

        let clock = Clock::get()?;
        let block_time = clock.unix_timestamp;

        self.event_emitter.emit_new_event(
            Some(block_time),
            LollysLottoProgramEventData::StartLottoGame(StartLottoGameEvent {
                round: lotto_game.round,
                round_name,
                game_duration,
                lotto_game_pubkey: lotto_game.key(),
                start_date: lotto_game.start_date,
                end_date: lotto_game.end_date,
                ticket_price: lotto_game.ticket_price,
                state: lotto_game.state,
            }),
        )?;
        Ok(())
    }
}
