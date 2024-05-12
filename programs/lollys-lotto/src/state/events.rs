use super::{LottoGameState, LottoGameVersion, LottoTicketNumbers};
use crate::pda_identifier::PDAIdentifier;
use anchor_lang::prelude::*;

const CURRENT_EVENT_VERSION: u8 = 0;

/// Tracker for event emission.
#[account]
#[derive(Debug, Default)]
pub struct EventEmitter {
    /// One-up, for tracking gaps in recorded program history
    pub event_id: i64,
}

impl PDAIdentifier for EventEmitter {
    const IDENT: &'static [u8] = b"event-emitter";

    #[inline(always)]
    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl EventEmitter {
    pub fn address() -> Pubkey {
        Self::get_address(&[])
    }

    pub fn address_with_bump() -> (Pubkey, u8) {
        Self::get_address_with_bump(&[])
    }
    /// [LollysLottoProgramEvent] factory function.
    /// All events should be created through this method, to ensure proper
    /// incrementing.
    pub fn emit_new_event(
        &mut self,
        block_time: Option<i64>,
        data: LollysLottoProgramEventData,
    ) -> Result<()> {
        let block_time = block_time.unwrap_or(Clock::get()?.unix_timestamp);
        let event = LollysLottoProgramEvent {
            event_id: self.event_id.clone(),
            version: CURRENT_EVENT_VERSION,
            block_time,
            data,
        };
        emit!(event);
        self.increment();
        Ok(())
    }

    fn increment(&mut self) {
        self.event_id += 1;
    }
}

// TODO Parse from 8 bytes, match against instruction discriminator
/// Used to decode the type of instruction that occurred, you can decode
/// this from a historical transaction's raw instruction data.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ProgramInstruction {
    //admin
    BurnLolly,
    CloseEventEmitter,
    CloseLollyBurnState,
    CloseLollysLotto,
    CloseLottoGame,
    CreateLollyBurnState,
    CreateLollysLotto,
    StartLottoGame,
    SwapUsdcLolly,
    // switchboard
    ProcessWinningNumbers,
    RequestWinningNumbers,
    TestEmitWinningNumbers,
    // user
    BuyLottoTicket,
    ClaimUserRewards,
    CloseLottoTicket,
    CloseUserMetadata,
    CreateUserMetadata,
    // cranks
    CrankLottoGameClosed,
    CrankLottoGameWinners,
    CrankTransferWinningAmountToUserRewardsVault,
}

/// Created with `EventSigner::new_event()`.
#[event]
pub struct LollysLottoProgramEvent {
    event_id: i64,
    version: u8,
    block_time: i64,
    pub data: LollysLottoProgramEventData,
}
impl LollysLottoProgramEvent {
    pub fn event_id(&self) -> i64 {
        self.event_id
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn block_time(&self) -> i64 {
        self.block_time
    }
}

/// The inner data of an [LollysLottoProgramEvent]
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum LollysLottoProgramEventData {
    BurnLolly(BurnLollyEvent),
    CloseEventEmitter(CloseEventEmitterEvent),
    CloseLollyBurnState(CloseLollyBurnStateEvent),
    CloseLollysLotto(CloseLollysLottoEvent),
    CloseLottoGame(CloseLottoGameEvent),
    CreateLollyBurnState(CreateLollyBurnStateEvent),
    CreateLollysLotto(CreateLollysLottoEvent),
    StartLottoGame(StartLottoGameEvent),
    SwapUsdcLolly(SwapUsdcLollyEvent),
    ProcessWinningNumbers(ProcessWinningNumbersEvent),
    RequestWinningNumbers(RequestWinningNumbersEvent),
    TestEmitWinningNumbers(TestEmitWinningNumbersEvent),
    BuyLottoTicket(BuyLottoTicketEvent),
    ClaimUserRewards(ClaimUserRewardsEvent),
    CloseLottoTicket(CloseLottoTicketEvent),
    CloseUserMetadata(CloseUserMetadataEvent),
    CreateUserMetadata(CreateUserMetadataEvent),
    CrankLottoGameClosed(CrankLottoGameClosedEvent),
    CrankLottoGameWinners(CrankLottoGameWinnersEvent),
    CrankTransferWinningAmountToUserRewardsVault(CrankTransferWinningAmountToUserRewardsVaultEvent),

    // Add new events here
    DuplicateWinningNumbers(DuplicateWinningNumbersEvent),
}

/// Event emitted when a user burns $LOLLY tokens.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BurnLollyEvent {
    pub authority: Pubkey,
    pub lolly_burn_state: Pubkey,
    pub lolly_burnt_amount_now: u64,
    pub total_lolly_burnt: u64,
}

/// Event emitted when a user closes EventEmitter state.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CloseEventEmitterEvent {
    pub event_emitter: Pubkey,
}

/// Event emitted when a user closes a LollyBurnState.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CloseLollyBurnStateEvent {
    pub lolly_burn_state: Pubkey,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CloseLollysLottoEvent {
    pub lollys_lotto: Pubkey,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CloseLottoGameEvent {
    pub lotto_game: Pubkey,
    pub round: u64,
}

/// Event emitted when a user creates a lolly burn state.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateLollyBurnStateEvent {
    pub authority: Pubkey,
    pub lolly_burn_state: Pubkey,
    pub lolly_mint: Pubkey,
    pub lolly_burn_state_lolly_vault: Pubkey,
    pub usdc_mint: Pubkey,
    pub lolly_burn_state_usdc_vault: Pubkey,
}

/// Event emitted when a user creates a lolly lotto.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateLollysLottoEvent {
    pub authority: Pubkey,
    pub lollys_lotto: Pubkey,
    pub lotto_game_count: u64,
}

/// Event emitted when a user starts a lotto game.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StartLottoGameEvent {
    pub lotto_game_version: LottoGameVersion,
    pub round: u64,
    pub round_name: String,
    pub game_duration: u64,
    pub authority: Pubkey,
    pub lotto_game_pubkey: Pubkey,
    pub lotto_game_vault: Pubkey,
    pub lotto_game_mint: Pubkey,
    pub start_date: i64,
    pub end_date: i64,
    pub ticket_price: u64,
    pub state: LottoGameState,
    pub lotto_game_count: u64,
}

/// Event emitted when a user swaps USDC for lolly.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SwapUsdcLollyEvent {
    pub authority: Pubkey,
    pub lolly_burn_state: Pubkey,
}

/// Event emitted when a user updates a lotto game.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CrankLottoGameClosedEvent {
    pub lotto_game: Pubkey,
    pub round: u64,
    pub ticket_price: u64,
    pub game_duration: u64,
}

/// Event emitted when a user consumes randomness.
/// This is a placeholder event, and will be replaced with a more
/// meaningful event in the future.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProcessWinningNumbersEvent {
    pub lotto_game: Pubkey,
    pub round: u64,
    pub randomness: Vec<u8>,
    pub winning_numbers: [u8; 6],
    pub winning_numbers_updated_index: [i64; 4],
}

/// Event emitted when duplicate winning numbers are detected.

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DuplicateWinningNumbersEvent {
    pub lotto_game: Pubkey,
    pub round: u64,
    pub randomness: Vec<u8>,
    pub duplicate_numbers: [u8; 6],
    pub duplicate_number_detected_index: [i64; 4],
}

/// Event emitted when a user requests randomness.

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RequestWinningNumbersEvent {
    pub lotto_game: Pubkey,
    pub round: u64,
}

/// Test Event emitted when a user requests randomness.

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TestEmitWinningNumbersEvent {
    pub round: u64,
    pub randomness: Vec<u8>,
}

/// Event emitted when a user buys a lotto ticket.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BuyLottoTicketEvent {
    pub user: Pubkey,
    pub user_metadata: Pubkey,
    pub user_ticket_count: u64,
    pub lotto_ticket: Pubkey,
    pub lotto_game: Pubkey,
    pub tickets_sold: u64,
    pub round: u64,
    pub ticket_number: u64,
    pub numbers: LottoTicketNumbers,
    pub ticket_price: u64,
    pub buy_date: i64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ClaimUserRewardsEvent {
    pub user: Pubkey,
    pub user_metadata: Pubkey,
    pub user_rewards_vault: Pubkey,
    pub amount_to_be_claimed: u64,
    pub total_amount_claimed: u64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CloseLottoTicketEvent {
    pub round: u64,
    pub numbers: [u8; 6],
    pub lotto_game: Pubkey,
    pub lotto_ticket: Pubkey,
    pub user: Pubkey,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CloseUserMetadataEvent {
    pub user_metadata: Pubkey,
}

/// Event emitted when a user creates user metadata.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateUserMetadataEvent {
    pub user: Pubkey,
    pub user_metadata: Pubkey,
    pub user_rewards_vault: Pubkey,
    pub created_timestamp: i64,
}

/// Event emitted when a admin cranks a lotto game winner.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CrankLottoGameWinnersEvent {
    pub round: u64,
    pub winning_numbers: LottoTicketNumbers,
    pub winning_numbers_index: [i64; 4],
    pub winning_user: Pubkey,
    pub lotto_ticket: Pubkey,
    pub lotto_game: Pubkey,
}

/// Event emitted when admin crank transfers winning amount to user rewards vault.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CrankTransferWinningAmountToUserRewardsVaultEvent {
    pub round: u64,
    pub winning_numbers: LottoTicketNumbers,
    pub number_of_tickets_with_duplicate_numbers: u32,
    pub lotto_game: Pubkey,
    pub user: Pubkey,
    pub lotto_ticket: Pubkey,
    pub winning_amount: u64,
}
