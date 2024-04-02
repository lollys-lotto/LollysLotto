pub use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod constants;
pub mod errors;
pub mod pda_identifier;
pub mod utils;

use instructions::*;


declare_id!("EQHT3TFXS3hBMzSpJiKb84sHE7iBXnYBpWvQTU8r91m6");

#[program]
pub mod lollys_lotto {
    use super::*;

    pub fn create_lollys_lotto(ctx: Context<CreateLollysLotto>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.lollys_lotto)
    }

    pub fn start_lotto_game(
        ctx: Context<StartLottoGame>, 
        round: u64, 
        ticket_price: u64, 
        game_duration: u64, 
        round_name: String,
    ) -> Result<()> {
        ctx.accounts.process(ctx.bumps.lotto_game, ctx.bumps.lotto_game_vault_signer, round, ticket_price, game_duration, round_name)
    }

    pub fn create_user_metadata(ctx: Context<CreateUserMetadata>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.user_metadata)
    }

    pub fn buy_lotto_ticket(ctx: Context<BuyLottoTicket>, round: u64, numbers: [u8; 6]) -> Result<()> {
        ctx.accounts.process(round, numbers)
    }
    
    pub fn create_lolly_burn_state(ctx: Context<CreateLollyBurnState>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.lolly_burn_state)
    }

    // pub fn crank_lotto_game_winner(ctx: Context<CrankLottoGameWinner>, round: u64, winning_numbers: [u8; 6]) -> Result<()> {
        pub fn crank_lotto_game_winner(ctx: Context<CrankLottoGameWinner>, winning_numbers: [u8; 6], winning_amount: u64) -> Result<()> {
        ctx.accounts.process(winning_numbers, winning_amount)
    }

    pub fn swap_usdc_lolly<'a, 'b, 'c:'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SwapUsdcLolly<'info>>, 
        data: Vec<u8>
    ) -> Result<()> {
        swap_usdc_lolly::swap_usdc_lolly(ctx, data)
    }

    pub fn burn_lolly(ctx: Context<BurnLolly>) -> Result<()> {
        burn_lolly::burn_lolly(ctx)
    }

    pub fn request_winning_numbers(ctx: Context<RequestWinningNumbers>) -> Result<()> {
        request_winning_numbers::request_winning_numbers(ctx)
    }

    pub fn process_winning_numbers(ctx: Context<ProcessWinningNumbers>, result: Vec<u8>) -> Result<()> {
        process_winning_numbers::process_winning_numbers(ctx, result)
    }

    pub fn test_emit_winning_numbers(ctx: Context<TestEmitWinningNumbers>, result: Vec<u8>) -> Result<()> {
        test_emit_winning_numbers::test_emit_winning_numbers(ctx, result)
    }

    pub fn create_event_emitter(ctx: Context<CreateEventEmitter>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn close_event_emitter(ctx: Context<CloseEventEmitter>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn close_lotto_game(ctx: Context<CloseLottoGame>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn close_lolly_lotto(ctx: Context<CloseLollyLotto>) -> Result<()> {
        ctx.accounts.process()
    }

}
