pub use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod pda_identifier;
pub mod state;
pub mod utils;

use instructions::*;
use state::LottoTicketNumbers;

declare_id!("13j1mUzsVqhSEiV3QP2oF2a4AUEQuhmQcjdF8nBrG1o1");

#[program]
pub mod lollys_lotto {

    use super::*;

    // Admin instructions
    pub fn burn_lolly(ctx: Context<BurnLolly>) -> Result<()> {
        burn_lolly::burn_lolly(ctx)
    }

    pub fn close_event_emitter(ctx: Context<CloseEventEmitter>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn close_lolly_burn_state(ctx: Context<CloseLollyBurnState>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn close_lollys_lotto(ctx: Context<CloseLollysLotto>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn close_lotto_game(ctx: Context<CloseLottoGame>) -> Result<()> {
        close_lotto_game::close_lotto_game(ctx)
    }

    pub fn create_event_emitter(ctx: Context<CreateEventEmitter>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn create_lolly_burn_state(ctx: Context<CreateLollyBurnState>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.lolly_burn_state)
    }
    pub fn create_lollys_lotto(ctx: Context<CreateLollysLotto>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.lollys_lotto)
    }

    pub fn start_lotto_game(
        ctx: Context<StartLottoGame>,
        round: u64,
        ticket_price: u64,
        game_duration: u64,
        randomness_account: Pubkey,
        round_name: String,
    ) -> Result<()> {
        start_lotto_game::start_lotto_game(ctx, round, ticket_price, game_duration, randomness_account, round_name)
    }

    pub fn swap_usdc_lolly<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SwapUsdcLolly<'info>>,
        data: Vec<u8>,
    ) -> Result<()> {
        swap_usdc_lolly::swap_usdc_lolly(ctx, data)
    }

    // Crank instructions

    pub fn crank_lotto_game_closed(ctx: Context<CrankLottoGameClosed>, round: u64) -> Result<()> {
        crank_lotto_game_closed::crank_lotto_game_closed(ctx, round)
    }

    pub fn crank_lotto_game_winners(
        ctx: Context<CrankLottoGameWinners>,
        round: u64,
        winning_numbers: LottoTicketNumbers,
        winning_numbers_index: [i64; 4],
    ) -> Result<()> {
        crank_lotto_game_winners::crank_lotto_game_winners(
            ctx,
            round,
            winning_numbers,
            winning_numbers_index,
        )
    }

    pub fn crank_transfer_winning_amount_to_user_rewards_vault(
        ctx: Context<CrankTransferWinningAmountToUserRewardsVault>,
        round: u64,
        winning_numbers: LottoTicketNumbers,
        number_of_tickets_with_duplicate_numbers: u32,
    ) -> Result<()> {
        crank_transfer_winning_amount_to_user_rewards_vault::crank_transfer_winning_amount_to_user_rewards_vault(
            ctx,
            round,
            winning_numbers,
            number_of_tickets_with_duplicate_numbers,
        )
    }

    pub fn crank_transfer_to_buy_and_burn_vault(
        ctx: Context<CrankTransferToBuyAndBurnVault>,
        round: u64,
    ) -> Result<()> {
        crank_transfer_to_buy_and_burn_vault::crank_transfer_to_buy_and_burn_vault(ctx, round)
    }

    // Switchboard instructions
    pub fn process_winning_numbers(
        ctx: Context<ProcessWinningNumbers>,
        // result: [u8; 32],
    ) -> Result<()> {
        process_winning_numbers::process_winning_numbers(ctx)
        // process_winning_numbers::process_winning_numbers(ctx, result)
    }

    pub fn request_winning_numbers(ctx: Context<RequestWinningNumbers>) -> Result<()> {
        request_winning_numbers::request_winning_numbers(ctx)
    }

    pub fn request_winning_numbers_v2(ctx: Context<RequestWinningNumbersV2>) -> Result<()> {
        request_winning_numbers_v2::request_winning_numbers_v2(ctx)
    }

    pub fn test_emit_winning_numbers(
        ctx: Context<TestEmitWinningNumbers>,
        result: [u8; 32],
    ) -> Result<()> {
        test_emit_winning_numbers::test_emit_winning_numbers(ctx, result)
    }

    // User instructions
    pub fn buy_lotto_ticket(
        ctx: Context<BuyLottoTicket>,
        round: u64,
        lotto_ticket_numbers: LottoTicketNumbers,
    ) -> Result<()> {
        ctx.accounts.process(round, lotto_ticket_numbers)
    }

    pub fn claim_user_rewards(
        ctx: Context<ClaimUserRewards>,
        amount_to_be_claimed: u64,
    ) -> Result<()> {
        ctx.accounts.process(amount_to_be_claimed)
    }

    pub fn close_lotto_ticket(
        ctx: Context<CloseLottoTicket>,
        round: u64,
        numbers: LottoTicketNumbers,
    ) -> Result<()> {
        ctx.accounts.process(round, numbers)
    }

    pub fn close_user_metadata(ctx: Context<CloseUserMetadata>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn create_user_metadata(ctx: Context<CreateUserMetadata>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.user_metadata)
    }
}
