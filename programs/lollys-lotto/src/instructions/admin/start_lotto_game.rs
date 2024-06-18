use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::USDC_MINT_DEVNET;
use crate::errors::LollysLottoError;
use crate::pda_identifier::PDAIdentifier;
use crate::state::{
    EventEmitter, LollysLotto, LollysLottoProgramEventData, LottoGame, LottoGameState,
    LottoGameVault, LottoGameVersion, LottoGameWinningNumbers, LottoTicketNumbers,
    StartLottoGameEvent,
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

pub fn start_lotto_game(
    ctx: Context<StartLottoGame>,
    round: u64,
    ticket_price: u64,
    game_duration: u64,
    randomness_account: Pubkey,
    round_name: String,
) -> Result<()> {
    let lotto_game = &mut *ctx.accounts.lotto_game.load_init()?;
    let lollys_lotto = &mut ctx.accounts.lollys_lotto;

    if round != lollys_lotto.lotto_game_count {
        return Err(LollysLottoError::RoundNumbersAreSequential.into());
    }
    
    lotto_game.bump = ctx.bumps.lotto_game;
    lotto_game.lotto_game_vault_bump = ctx.bumps.lotto_game_vault_signer;
    lotto_game.version = LottoGameVersion::V1;
    lotto_game.state = LottoGameState::Open;
    lotto_game.authority = *ctx.accounts.authority.key;
    lotto_game.round = lollys_lotto.lotto_game_count;
    lotto_game.start_date = Clock::get()?.unix_timestamp;
    lotto_game.end_date = lotto_game.start_date + game_duration as i64;
    lotto_game.ticket_price = ticket_price;
    lotto_game.tickets_sold = 0;
    lotto_game.lotto_game_mint = *ctx.accounts.lotto_game_mint.to_account_info().key;
    lotto_game.lotto_game_vault = *ctx.accounts.lotto_game_vault.to_account_info().key;
    lotto_game.jackpot_winning_ticket = Pubkey::default();
    lotto_game.max_numbers_in_ticket = [0; 6];
    lotto_game._padding1 = [0; 2];
    lotto_game.max_numbers_in_ticket[0] = LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number1;
    lotto_game.max_numbers_in_ticket[1] = LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number2;
    lotto_game.max_numbers_in_ticket[2] = LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number3;
    lotto_game.max_numbers_in_ticket[3] = LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number4;
    lotto_game.max_numbers_in_ticket[4] = LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.number5;
    lotto_game.max_numbers_in_ticket[5] =
        LottoTicketNumbers::MAX_NUMBERS_IN_TICKET_V1.jackpot_number;
    lotto_game.randomness_account = randomness_account;
    lotto_game.jackpot_winning_numbers = LottoGameWinningNumbers::default();
    lotto_game.tier_1_winning_numbers =
        [LottoGameWinningNumbers::default(); LottoGame::MAX_TIER_1_WINNERS_V1];
    lotto_game.tier_2_winning_numbers =
        [LottoGameWinningNumbers::default(); LottoGame::MAX_TIER_2_WINNERS_V1];
    lotto_game.tier_3_winning_numbers =
        [LottoGameWinningNumbers::default(); LottoGame::MAX_TIER_3_WINNERS_V1];

    lollys_lotto.lotto_game_count += 1;

    let block_time = Clock::get()?.unix_timestamp;
    ctx.accounts.event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::StartLottoGame(StartLottoGameEvent {
            lotto_game_version: lotto_game.version,
            round: lotto_game.round,
            round_name,
            game_duration,
            authority: lotto_game.authority,
            lotto_game_pubkey: ctx.accounts.lotto_game.key(),
            lotto_game_vault: ctx.accounts.lotto_game_vault.key(),
            lotto_game_mint: ctx.accounts.lotto_game_mint.key(),
            start_date: lotto_game.start_date,
            end_date: lotto_game.end_date,
            ticket_price: lotto_game.ticket_price,
            state: lotto_game.state,
            lotto_game_count: lollys_lotto.lotto_game_count,
        }),
    )?;
    Ok(())
}
