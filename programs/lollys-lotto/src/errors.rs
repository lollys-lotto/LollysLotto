use anchor_lang::error::ERROR_CODE_OFFSET;
use anchor_lang::error_code;

#[error_code]
pub enum LollyError {
    #[msg("Overflow")]
    OverflowError,

    #[msg("Source token account of jupiter ix mismatch")]
    JupiterIxSourceTokenAccountMismatch,

    #[msg("Destination token account of jupiter ix mismatch")]
    JupiterIxDestinationTokenAccountMismatch,

    #[msg("Only $LOLLY tokens are allowed to burn!")]
    OnlyLOLLYBuringAllowed,

    #[msg("Token Account authority mismatch!")]
    TokenAccountAuthorityMismatch,

    #[msg("Only Swap to $LOLLY tokens are allowed")]
    OnlySwapToLOLLYAllowed,

    #[msg("Only Swap from $USDC tokens are allowed")]
    OnlySwapFromUSDCAllowed,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Lotto Game not open")]
    LottoGameNotOpen,

    #[msg("Invalid round")]
    InvalidRound,

    #[msg("Invalid winning ticket")]
    InvalidWinningTicket,

    #[msg("Ticket is already declared winner")]
    AlreadyDeclaredWinner,

    #[msg("Game not closed")]
    GameNotClosed,

    #[msg("Round numbers have to be sequential")]
    RoundNumbersAreSequential,

    #[msg("Lotto Game has ended")]
    LottoGameEnded,

    #[msg("Game not finished")]
    GameNotFinished,

    #[msg("No rewards to claim from vault")]
    NoRewardsToClaimFromVault,

    #[msg("Not sufficient rewards in vault")]
    NotSufficientRewardsInVault,
}

pub const NUM_ERR_VARIANTS: u32 = 18;

impl TryInto<LollyError> for u32 {
    // If the u32 is not within the bounds of [ERROR_CODE_OFFSET] and
    // [ERROR_CODE_OFFSET + NUM_ERR_VARIANTS, this error is returned.
    type Error = ();

    fn try_into(self) -> Result<LollyError, ()> {
        if (ERROR_CODE_OFFSET..=ERROR_CODE_OFFSET + NUM_ERR_VARIANTS).contains(&self) {
            Ok(unsafe { std::mem::transmute(self - ERROR_CODE_OFFSET) })
        } else {
            Err(())
        }
    }
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum RandomnessRequestError {
    #[msg("Invalid authority account")]
    InvalidAuthority,
    #[msg("Invalid escrow account")]
    InvalidEscrow,
    #[msg("Array overflow")]
    ArrayOverflow,
    #[msg("Stale data")]
    StaleData,
    #[msg("Invalid trusted signer")]
    InvalidTrustedSigner,
    #[msg("Invalid MRENCLAVE")]
    InvalidMrEnclave,
    #[msg("Failed to find a valid trading symbol for this price")]
    InvalidSymbol,
    #[msg("FunctionAccount pubkey did not match program_state.function")]
    IncorrectSwitchboardFunction,
    #[msg("FunctionAccount pubkey did not match program_state.function")]
    InvalidSwitchboardFunction,
    #[msg("FunctionAccount was not validated successfully")]
    FunctionValidationFailed,
    #[msg("FunctionRequestAccount status should be 'RequestSuccess'")]
    SwitchboardRequestNotSuccessful,
    #[msg("Round is inactive")]
    RoundInactive,
    #[msg("House has insufficient funds to payout winners")]
    HouseInsufficientFunds,
}
