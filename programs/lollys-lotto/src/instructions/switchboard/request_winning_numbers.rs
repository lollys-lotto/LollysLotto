use anchor_lang::prelude::*;
use solana_randomness_service::program::SolanaRandomnessService;
use solana_randomness_service::TransactionOptions;
use switchboard_solana::{utils::get_ixn_discriminator, NativeMint};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token},
};

use crate::pda_identifier::PDAIdentifier;
use crate::state::{
    EventEmitter, LollysLottoProgramEventData, LottoGame, RequestWinningNumbersEvent,
};

#[derive(Accounts)]
pub struct RequestWinningNumbers<'info> {
    /// The Solana Randomness Service program.
    pub randomness_service: Program<'info, SolanaRandomnessService>,

    /// The account that will be created on-chain to hold the randomness request.
    /// Used by the off-chain oracle to pickup the request and fulfill it.
    /// CHECK: todo
    #[account(
        mut,
        signer,
        owner = system_program.key(),
        constraint = randomness_request.data_len() == 0 && randomness_request.lamports() == 0,
    )]
    pub randomness_request: AccountInfo<'info>,

    /// The TokenAccount that will store the funds for the randomness request.
    /// CHECK: todo
    #[account(
        mut,
        owner = system_program.key(),
        constraint = randomness_escrow.data_len() == 0 && randomness_escrow.lamports() == 0,
    )]
    pub randomness_escrow: AccountInfo<'info>,

    /// The randomness service's state account. Responsible for storing the
    /// reward escrow and the cost per random byte.
    #[account(
        seeds = [b"STATE"],
        bump = randomness_state.bump,
        seeds::program = randomness_service.key(),
    )]
    pub randomness_state: Box<Account<'info, solana_randomness_service::State>>,

    /// The token mint to use for paying for randomness requests.
    #[account(address = NativeMint::ID)]
    pub randomness_mint: Account<'info, Mint>,

    /// The account that will pay for the randomness request.
    #[account(mut)]
    pub payer: Signer<'info>,

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

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    /// The Solana System program. Used to allocate space on-chain for the randomness_request account.
    pub system_program: Program<'info, System>,

    /// The Solana Token program. Used to transfer funds to the randomness escrow.
    pub token_program: Program<'info, Token>,

    /// The Solana Associated Token program. Used to create the TokenAccount for the randomness escrow.
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn request_winning_numbers(
    ctx: Context<RequestWinningNumbers>,
) -> anchor_lang::prelude::Result<()> {
    msg!("Requesting randomness...");

    // Call the randomness service and request a new value
    solana_randomness_service::cpi::simple_randomness_v1(
        CpiContext::new(
            ctx.accounts.randomness_service.to_account_info(),
            solana_randomness_service::cpi::accounts::SimpleRandomnessV1Request {
                request: ctx.accounts.randomness_request.to_account_info(),
                escrow: ctx.accounts.randomness_escrow.to_account_info(),
                state: ctx.accounts.randomness_state.to_account_info(),
                mint: ctx.accounts.randomness_mint.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            },
        ),
        6, // Request 6 bytes of randomness
        solana_randomness_service::Callback {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new_readonly(ctx.accounts.randomness_state.key(), true).into(),
                AccountMeta::new_readonly(ctx.accounts.randomness_request.key(), false).into(),
                AccountMeta::new_readonly(ctx.accounts.authority.key(), false).into(),
                AccountMeta::new_readonly(ctx.accounts.lotto_game.key(), false).into(),
                AccountMeta::new(ctx.accounts.event_emitter.key(), false).into(),
            ],
            ix_data: get_ixn_discriminator("process_winning_numbers").to_vec(), // TODO: hardcode this discriminator [190,217,49,162,99,26,73,234]
        },
        Some(TransactionOptions {
            compute_units: Some(1_000_000),
            compute_unit_price: Some(100),
        }),
    )?;
    // Here we can emit some event to index our requests
    let event_emitter = &mut ctx.accounts.event_emitter;
    let lotto_game = &mut ctx.accounts.lotto_game;
    let block_time = Clock::get()?.unix_timestamp;
    event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::RequestWinningNumbers(RequestWinningNumbersEvent {
            lotto_game: lotto_game.key(),
            round: lotto_game.load()?.round,
        }),
    )?;

    Ok(())
}
