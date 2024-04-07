use super::LottoGameState;
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
    ProcessWinningNumbers,
    RequestWinningNumbers,
    BurnLolly,
    BuyLottoTicket,
    CrankLottoGameWinner,
    CreateLollyBurnState,
    CreateLollysLotto,
    CreateUserMetadata,
    StartLottoGame,
    SwapUsdcLolly,
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
    BuyLottoTicket(BuyLottoTicketEvent),
    ClaimUserRewards(ClaimUserRewardsEvent),
    CloseEventEmitter(CloseEventEmitterEvent),
    CloseLollysLotto(CloseLollysLottoEvent),
    CloseLottoGame(CloseLottoGameEvent),
    CrankLottoGameWinner(CrankLottoGameWinnerEvent),
    CreateLollyBurnState(CreateLollyBurnStateEvent),
    CreateLollysLotto(CreateLollysLottoEvent),
    CreateUserMetadata(CreateUserMetadataEvent),
    StartLottoGame(StartLottoGameEvent),
    SwapUsdcLolly(SwapUsdcLollyEvent),
    ProcessWinningNumbers(ProcessWinningNumbersEvent),
    RequestWinningNumbers(RequestWinningNumbersEvent),
    TestEmitWinningNumbers(TestEmitWinningNumbersEvent),
}

/// Event emitted when a user burns a lolly.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BurnLollyEvent {
    pub user: Pubkey,
    pub lolly: Pubkey,
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
    pub numbers: [u8; 6],
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ClaimUserRewardsEvent {
    pub user: Pubkey,
    pub user_metadata: Pubkey,
    pub user_rewards_vault: Pubkey,
    pub amount_to_be_claimed: u64,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CloseEventEmitterEvent {
    pub event_emitter: Pubkey,
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
/// Event emitted when a user cranks a lotto game winner.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CrankLottoGameWinnerEvent {
    pub user: Pubkey,
    pub game: Pubkey,
    pub winner: Pubkey,
}

/// Event emitted when a user creates a lolly burn state.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateLollyBurnStateEvent {
    pub user: Pubkey,
    pub lolly_burn_state: Pubkey,
}

/// Event emitted when a user creates a lolly lotto.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateLollysLottoEvent {
    pub authority: Pubkey,
    pub lollys_lotto: Pubkey,
    pub lotto_game_count: u64,
}

/// Event emitted when a user creates user metadata.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateUserMetadataEvent {
    pub user: Pubkey,
    pub user_metadata: Pubkey,
    pub created_timestamp: i64,
}

/// Event emitted when a user starts a lotto game.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StartLottoGameEvent {
    pub round: u64,
    pub round_name: String,
    pub game_duration: u64,
    pub lotto_game_pubkey: Pubkey,
    pub start_date: i64,
    pub end_date: i64,
    pub ticket_price: u64,
    pub state: LottoGameState,
}

/// Event emitted when a user swaps USDC for lolly.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SwapUsdcLollyEvent {
    pub user: Pubkey,
    pub usdc: Pubkey,
    pub lolly: Pubkey,
}

/// Event emitted when a user consumes randomness.
/// This is a placeholder event, and will be replaced with a more
/// meaningful event in the future.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProcessWinningNumbersEvent {
    pub round: u64,
    pub randomness: Vec<u8>,
}

/// Event emitted when a user requests randomness.

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RequestWinningNumbersEvent {
    pub round: u64,
}

/// Test Event emitted when a user requests randomness.

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TestEmitWinningNumbersEvent {
    pub round: u64,
    pub randomness: Vec<u8>,
}
