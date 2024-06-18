use anchor_lang::error::ERROR_CODE_OFFSET;
use anchor_lang::error_code;
use std::{convert::TryInto, fmt::Display};

pub type LollysLottoResult<T> = std::result::Result<T, LollysLottoError>;

// NOTE: We cannot use ERROR_CODE_OFFSET because it expects an integer literal
/// Custom error code: 6100 + idx => 0x17D4 + 0x${idx}
#[error_code(offset = 6100)]
#[derive(PartialEq)]
pub enum LollysLottoError {
    #[msg("[L100] Overflow")] //0x17D4 (6100)
    OverflowError,

    #[msg("[L101] Source token account of jupiter ix mismatch")] //0x17D5 (6101)
    JupiterIxSourceTokenAccountMismatch,

    #[msg("[L102] Destination token account of jupiter ix mismatch")] //0x17D6 (6102)
    JupiterIxDestinationTokenAccountMismatch,

    #[msg("[L103] Only $LOLLY tokens are allowed to burn!")] //0x17D7 (6103)
    OnlyLOLLYBuringAllowed,

    #[msg("[L104] Token Account authority mismatch!")] //0x17D8 (6104)
    TokenAccountAuthorityMismatch,

    #[msg("[L105] Only Swap to $LOLLY tokens are allowed")] //0x17D9 (6105)
    OnlySwapToLOLLYAllowed,

    #[msg("[L106] Only Swap from $USDC tokens are allowed")] //0x17DA (6106)
    OnlySwapFromUSDCAllowed,

    #[msg("[L107] Insufficient funds")] //0x17DB (6107)
    InsufficientFunds,

    #[msg("[L108] Lotto Game not open")] //0x17DC (6108)
    LottoGameNotOpen,

    #[msg("[L109] Invalid round")] //0x17DD (6109)
    InvalidRound,

    #[msg("[L110] Invalid winning ticket")] //0x17DE (6110)
    InvalidWinningTicket,

    #[msg("[L111] Ticket is already declared winner")] //0x17DF (6111)
    AlreadyDeclaredWinner,

    #[msg("[L112] Game not closed")] //0x17E0 (6112)
    GameNotClosed,

    #[msg("[L113] Round numbers have to be sequential")] //0x17E1 (6113)
    RoundNumbersAreSequential,

    #[msg("[L114] Lotto Game has ended")] //0x17E2 (6114)
    LottoGameEnded,

    #[msg("[L115] Lotto Game is still open")] //0x17E3 (6115)
    LottoGameIsStillOpen,

    #[msg("[L116] No rewards to claim from vault")] //0x17E4 (6116)
    NoRewardsToClaimFromVault,

    #[msg("[L117] Not sufficient rewards in vault")] //0x17E5 (6117)
    NotSufficientRewardsInVault,

    #[msg("[L118] Lotto Game vault is not empty")] //0x17E6 (6118)
    LottoGameVaultNotEmpty,

    #[msg("[L119] Invalid numbers in ticket")] //0x17E7 (6119)
    InvalidNumbersInTicket,

    #[msg("[L120] Duplicate winning numbers")] //0x17E8 (6120)
    DuplicateWinningNumbers,

    #[msg("[L121] Winning number index is not provided")] //0x17E9 (6121)
    WinningNumberIndexIsNotProvided,

    #[msg("[L122] Winning numbers not set")] //0x17EA (6122)
    WinningNumbersNotSet,

    #[msg("[L123] Math Error")] //0x17EB (6123)
    MathError,

    #[msg("[L124] Invalid ticket")] //0x17EC (6124)
    NoAvailableSlots,

    #[msg("[L125] Invalid winning Index numbers")] //0x17ED (6125)
    InvalidWinningNumberIndex,

    #[msg("[L126] Invalid Crank Accounts")] //0x17EE (6126)
    InvalidCrankAccounts,

    #[msg("[L127] Invalid User Metadata PDA")] //0x17EF (6127)
    InvalidUserMetadataPDA,

    #[msg("[L128] Invalid User Rewards Vault PDA")] //0x17F0 (6128)
    InvalidUserRewardsVaultPDA,

    #[msg("[L129] Invalid Lotto Ticket PDA")] //0x17F1 (6129)
    InvalidLottoTicketPDA,

    #[msg("[L130] Jackpot Winning numbers not updated")] //0x17F2 (6130)
    JackpotWinningNumbersNotUpdated,

    #[msg("[L131] Jackpot amount already disbursed")] //0x17F3 (6131)
    JackpotAmountAlreadyDisbursed,

    #[msg("[L132] Tier 1 Winning number not updated")] //0x17F4 (6132)
    Tier1WinningNumbersNotUpdated,

    #[msg("[L133] Tier 1 amount already disbursed")] //0x17F5 (6133)
    Tier1AmountAlreadyDisbursed,

    #[msg("[L134] Tier 2 Winning number not updated")] //0x17F6 (6134)
    Tier2WinningNumbersNotUpdated,

    #[msg("[L135] Tier 2 amount already disbursed")] //0x17F7 (6135)
    Tier2AmountAlreadyDisbursed,

    #[msg("[L136] Tier 3 Winning number not updated")] //0x17F8 (6136)
    Tier3WinningNumbersNotUpdated,

    #[msg("[L137] Tier 3 amount already disbursed")] //0x17F9 (6137)
    Tier3AmountAlreadyDisbursed,

    #[msg("[L138] Invalid winning tier")] //0x17FA (6138)
    InvalidWinningTier,

    #[msg("[L139] No Duplicate tickets found")] //0x17FB (6139)
    NoDuplicateTicketsFound,

    #[msg("[L140] On Demad Randomness not resolved")] //0x17FC (6140)
    OnDemandRandomnessNotResolved,

    #[msg("[L141] Game Already Closed")] //0x17FD (6141)
    GameAlreadyClosed,

}

pub const LOLLY_NUM_ERR_VARIANTS: u32 = 41;

impl LollysLottoError {
    /// For use during checked math operations,
    /// logs the input values that caused the error.
    #[allow(unused_variables)]
    #[inline]
    pub fn with_operands<T: Display>(self, op1: T, op2: T) -> Self {
        #[cfg(feature = "debug-msg")]
        msg!("{}, {}", op1, op2);
        #[cfg(feature = "log")]
        log::error!("{}, {}", op1, op2);
        self
    }

    /// Use in `Result::map_err`.
    #[allow(unused_variables)]
    #[inline]
    pub fn with_cause(self, cause: impl Display) -> Self {
        #[cfg(feature = "debug-msg")]
        msg!("{}", cause);
        #[cfg(feature = "log")]
        log::error!("{}", cause);
        self
    }
}

impl TryInto<LollysLottoError> for u32 {
    // If the u32 is not within the bounds of [ERROR_CODE_OFFSET] and
    // [ERROR_CODE_OFFSET + LOLLY_NUM_ERR_VARIANTS, this error is returned.
    type Error = ();

    fn try_into(self) -> Result<LollysLottoError, ()> {
        if (ERROR_CODE_OFFSET..=ERROR_CODE_OFFSET + LOLLY_NUM_ERR_VARIANTS).contains(&self) {
            Ok(unsafe { std::mem::transmute(self - ERROR_CODE_OFFSET) })
        } else {
            Err(())
        }
    }
}

// NOTE: We cannot use RANDOMNESS_REQUEST_ERROR_CODE_OFFSET because it expects an integer literal
/// Custom error code: 7100 + idx => 0x17D4 + 0x${idx}
#[error_code(offset = 7100)]
#[derive(Eq, PartialEq)]
pub enum RandomnessRequestError {
    #[msg("[RR100] Invalid authority account")] //0x1BC4 (7100)
    InvalidAuthority,
    #[msg("[RR101] Invalid escrow account")] //0x1BC5 (7101)
    InvalidEscrow,
    #[msg("[RR102] Array overflow")] //0x1BC6 (7102)
    ArrayOverflow,
    #[msg("[RR103] Stale data")] //0x1BC7 (7103)
    StaleData,
    #[msg("[RR104] Invalid trusted signer")] //0x1BC8 (7104)
    InvalidTrustedSigner,
    #[msg("[RR105] Invalid MRENCLAVE")] //0x1BC9 (7105)
    InvalidMrEnclave,
    #[msg("[RR106] Failed to find a valid trading symbol for this price")] //0x1BCA (7106)
    InvalidSymbol,
    #[msg("[RR107] FunctionAccount pubkey did not match program_state.function")] //0x1BCB (7107)
    IncorrectSwitchboardFunction,
    #[msg("[RR108] FunctionAccount pubkey did not match program_state.function")] //0x1BCC (7108)
    InvalidSwitchboardFunction,
    #[msg("[RR109] FunctionAccount was not validated successfully")] //0x1BCD (7109)
    FunctionValidationFailed,
    #[msg("[RR110] FunctionRequestAccount status should be 'RequestSuccess'")] //0x1BCE (7110)
    SwitchboardRequestNotSuccessful,
    #[msg("[RR111] Round is inactive")] //0x1BCF (7111)
    RoundInactive,
    #[msg("[RR112] House has insufficient funds to payout winners")] //0x1BD0 (7112)
    HouseInsufficientFunds,
}

pub const RANDOMNESS_REQUEST_NUM_ERR_VARIANTS: u32 = 12;

impl RandomnessRequestError {
    /// For use during checked math operations,
    /// logs the input values that caused the error.
    #[allow(unused_variables)]
    #[inline]
    pub fn with_operands<T: Display>(self, op1: T, op2: T) -> Self {
        #[cfg(feature = "debug-msg")]
        msg!("{}, {}", op1, op2);
        #[cfg(feature = "log")]
        log::error!("{}, {}", op1, op2);
        self
    }

    /// Use in `Result::map_err`.
    #[allow(unused_variables)]
    #[inline]
    pub fn with_cause(self, cause: impl Display) -> Self {
        #[cfg(feature = "debug-msg")]
        msg!("{}", cause);
        #[cfg(feature = "log")]
        log::error!("{}", cause);
        self
    }
}
pub const RANDOMNESS_REQUEST_ERROR_CODE_OFFSET: u32 = 7000;
impl TryInto<RandomnessRequestError> for u32 {
    // If the u32 is not within the bounds of [RANDOMNESS_REQUEST_ERROR_CODE_OFFSET] and
    // [RANDOMNESS_REQUEST_ERROR_CODE_OFFSET + RANDOMNESS_REQUEST_NUM_ERR_VARIANTS, this error is returned.
    type Error = ();

    fn try_into(self) -> Result<RandomnessRequestError, ()> {
        if (RANDOMNESS_REQUEST_ERROR_CODE_OFFSET
            ..=RANDOMNESS_REQUEST_ERROR_CODE_OFFSET + RANDOMNESS_REQUEST_NUM_ERR_VARIANTS)
            .contains(&self)
        {
            Ok(unsafe { std::mem::transmute(self - RANDOMNESS_REQUEST_ERROR_CODE_OFFSET) })
        } else {
            Err(())
        }
    }
}
