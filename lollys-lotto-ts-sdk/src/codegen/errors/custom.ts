export type CustomError =
  | OverflowError
  | JupiterIxSourceTokenAccountMismatch
  | JupiterIxDestinationTokenAccountMismatch
  | OnlyLOLLYBuringAllowed
  | TokenAccountAuthorityMismatch
  | OnlySwapToLOLLYAllowed
  | OnlySwapFromUSDCAllowed
  | InsufficientFunds
  | LottoGameNotOpen
  | InvalidRound
  | InvalidWinningTicket
  | AlreadyDeclaredWinner
  | GameNotClosed
  | RoundNumbersAreSequential
  | LottoGameEnded
  | LottoGameIsStillOpen
  | NoRewardsToClaimFromVault
  | NotSufficientRewardsInVault
  | LottoGameVaultNotEmpty
  | InvalidNumbersInTicket
  | DuplicateWinningNumbers
  | WinningNumberIndexIsNotProvided
  | WinningNumbersNotSet
  | MathError
  | NoAvailableSlots
  | InvalidWinningNumberIndex
  | InvalidCrankAccounts
  | InvalidUserMetadataPDA
  | InvalidUserRewardsVaultPDA
  | InvalidLottoTicketPDA
  | JackpotWinningNumbersNotUpdated
  | JackpotAmountAlreadyDisbursed
  | Tier1WinningNumbersNotUpdated
  | Tier1AmountAlreadyDisbursed
  | Tier2WinningNumbersNotUpdated
  | Tier2AmountAlreadyDisbursed
  | Tier3WinningNumbersNotUpdated
  | Tier3AmountAlreadyDisbursed
  | InvalidWinningTier
  | NoDuplicateTicketsFound
  | OnDemandRandomnessNotResolved

export class OverflowError extends Error {
  static readonly code = 6000
  readonly code = 6000
  readonly name = "OverflowError"
  readonly msg = "[L100] Overflow"

  constructor(readonly logs?: string[]) {
    super("6000: [L100] Overflow")
  }
}

export class JupiterIxSourceTokenAccountMismatch extends Error {
  static readonly code = 6001
  readonly code = 6001
  readonly name = "JupiterIxSourceTokenAccountMismatch"
  readonly msg = "[L101] Source token account of jupiter ix mismatch"

  constructor(readonly logs?: string[]) {
    super("6001: [L101] Source token account of jupiter ix mismatch")
  }
}

export class JupiterIxDestinationTokenAccountMismatch extends Error {
  static readonly code = 6002
  readonly code = 6002
  readonly name = "JupiterIxDestinationTokenAccountMismatch"
  readonly msg = "[L102] Destination token account of jupiter ix mismatch"

  constructor(readonly logs?: string[]) {
    super("6002: [L102] Destination token account of jupiter ix mismatch")
  }
}

export class OnlyLOLLYBuringAllowed extends Error {
  static readonly code = 6003
  readonly code = 6003
  readonly name = "OnlyLOLLYBuringAllowed"
  readonly msg = "[L103] Only $LOLLY tokens are allowed to burn!"

  constructor(readonly logs?: string[]) {
    super("6003: [L103] Only $LOLLY tokens are allowed to burn!")
  }
}

export class TokenAccountAuthorityMismatch extends Error {
  static readonly code = 6004
  readonly code = 6004
  readonly name = "TokenAccountAuthorityMismatch"
  readonly msg = "[L104] Token Account authority mismatch!"

  constructor(readonly logs?: string[]) {
    super("6004: [L104] Token Account authority mismatch!")
  }
}

export class OnlySwapToLOLLYAllowed extends Error {
  static readonly code = 6005
  readonly code = 6005
  readonly name = "OnlySwapToLOLLYAllowed"
  readonly msg = "[L105] Only Swap to $LOLLY tokens are allowed"

  constructor(readonly logs?: string[]) {
    super("6005: [L105] Only Swap to $LOLLY tokens are allowed")
  }
}

export class OnlySwapFromUSDCAllowed extends Error {
  static readonly code = 6006
  readonly code = 6006
  readonly name = "OnlySwapFromUSDCAllowed"
  readonly msg = "[L106] Only Swap from $USDC tokens are allowed"

  constructor(readonly logs?: string[]) {
    super("6006: [L106] Only Swap from $USDC tokens are allowed")
  }
}

export class InsufficientFunds extends Error {
  static readonly code = 6007
  readonly code = 6007
  readonly name = "InsufficientFunds"
  readonly msg = "[L107] Insufficient funds"

  constructor(readonly logs?: string[]) {
    super("6007: [L107] Insufficient funds")
  }
}

export class LottoGameNotOpen extends Error {
  static readonly code = 6008
  readonly code = 6008
  readonly name = "LottoGameNotOpen"
  readonly msg = "[L108] Lotto Game not open"

  constructor(readonly logs?: string[]) {
    super("6008: [L108] Lotto Game not open")
  }
}

export class InvalidRound extends Error {
  static readonly code = 6009
  readonly code = 6009
  readonly name = "InvalidRound"
  readonly msg = "[L109] Invalid round"

  constructor(readonly logs?: string[]) {
    super("6009: [L109] Invalid round")
  }
}

export class InvalidWinningTicket extends Error {
  static readonly code = 6010
  readonly code = 6010
  readonly name = "InvalidWinningTicket"
  readonly msg = "[L110] Invalid winning ticket"

  constructor(readonly logs?: string[]) {
    super("6010: [L110] Invalid winning ticket")
  }
}

export class AlreadyDeclaredWinner extends Error {
  static readonly code = 6011
  readonly code = 6011
  readonly name = "AlreadyDeclaredWinner"
  readonly msg = "[L111] Ticket is already declared winner"

  constructor(readonly logs?: string[]) {
    super("6011: [L111] Ticket is already declared winner")
  }
}

export class GameNotClosed extends Error {
  static readonly code = 6012
  readonly code = 6012
  readonly name = "GameNotClosed"
  readonly msg = "[L112] Game not closed"

  constructor(readonly logs?: string[]) {
    super("6012: [L112] Game not closed")
  }
}

export class RoundNumbersAreSequential extends Error {
  static readonly code = 6013
  readonly code = 6013
  readonly name = "RoundNumbersAreSequential"
  readonly msg = "[L113] Round numbers have to be sequential"

  constructor(readonly logs?: string[]) {
    super("6013: [L113] Round numbers have to be sequential")
  }
}

export class LottoGameEnded extends Error {
  static readonly code = 6014
  readonly code = 6014
  readonly name = "LottoGameEnded"
  readonly msg = "[L114] Lotto Game has ended"

  constructor(readonly logs?: string[]) {
    super("6014: [L114] Lotto Game has ended")
  }
}

export class LottoGameIsStillOpen extends Error {
  static readonly code = 6015
  readonly code = 6015
  readonly name = "LottoGameIsStillOpen"
  readonly msg = "[L115] Lotto Game is still open"

  constructor(readonly logs?: string[]) {
    super("6015: [L115] Lotto Game is still open")
  }
}

export class NoRewardsToClaimFromVault extends Error {
  static readonly code = 6016
  readonly code = 6016
  readonly name = "NoRewardsToClaimFromVault"
  readonly msg = "[L116] No rewards to claim from vault"

  constructor(readonly logs?: string[]) {
    super("6016: [L116] No rewards to claim from vault")
  }
}

export class NotSufficientRewardsInVault extends Error {
  static readonly code = 6017
  readonly code = 6017
  readonly name = "NotSufficientRewardsInVault"
  readonly msg = "[L117] Not sufficient rewards in vault"

  constructor(readonly logs?: string[]) {
    super("6017: [L117] Not sufficient rewards in vault")
  }
}

export class LottoGameVaultNotEmpty extends Error {
  static readonly code = 6018
  readonly code = 6018
  readonly name = "LottoGameVaultNotEmpty"
  readonly msg = "[L118] Lotto Game vault is not empty"

  constructor(readonly logs?: string[]) {
    super("6018: [L118] Lotto Game vault is not empty")
  }
}

export class InvalidNumbersInTicket extends Error {
  static readonly code = 6019
  readonly code = 6019
  readonly name = "InvalidNumbersInTicket"
  readonly msg = "[L119] Invalid numbers in ticket"

  constructor(readonly logs?: string[]) {
    super("6019: [L119] Invalid numbers in ticket")
  }
}

export class DuplicateWinningNumbers extends Error {
  static readonly code = 6020
  readonly code = 6020
  readonly name = "DuplicateWinningNumbers"
  readonly msg = "[L120] Duplicate winning numbers"

  constructor(readonly logs?: string[]) {
    super("6020: [L120] Duplicate winning numbers")
  }
}

export class WinningNumberIndexIsNotProvided extends Error {
  static readonly code = 6021
  readonly code = 6021
  readonly name = "WinningNumberIndexIsNotProvided"
  readonly msg = "[L121] Winning number index is not provided"

  constructor(readonly logs?: string[]) {
    super("6021: [L121] Winning number index is not provided")
  }
}

export class WinningNumbersNotSet extends Error {
  static readonly code = 6022
  readonly code = 6022
  readonly name = "WinningNumbersNotSet"
  readonly msg = "[L122] Winning numbers not set"

  constructor(readonly logs?: string[]) {
    super("6022: [L122] Winning numbers not set")
  }
}

export class MathError extends Error {
  static readonly code = 6023
  readonly code = 6023
  readonly name = "MathError"
  readonly msg = "[L123] Math Error"

  constructor(readonly logs?: string[]) {
    super("6023: [L123] Math Error")
  }
}

export class NoAvailableSlots extends Error {
  static readonly code = 6024
  readonly code = 6024
  readonly name = "NoAvailableSlots"
  readonly msg = "[L124] Invalid ticket"

  constructor(readonly logs?: string[]) {
    super("6024: [L124] Invalid ticket")
  }
}

export class InvalidWinningNumberIndex extends Error {
  static readonly code = 6025
  readonly code = 6025
  readonly name = "InvalidWinningNumberIndex"
  readonly msg = "[L125] Invalid winning Index numbers"

  constructor(readonly logs?: string[]) {
    super("6025: [L125] Invalid winning Index numbers")
  }
}

export class InvalidCrankAccounts extends Error {
  static readonly code = 6026
  readonly code = 6026
  readonly name = "InvalidCrankAccounts"
  readonly msg = "[L126] Invalid Crank Accounts"

  constructor(readonly logs?: string[]) {
    super("6026: [L126] Invalid Crank Accounts")
  }
}

export class InvalidUserMetadataPDA extends Error {
  static readonly code = 6027
  readonly code = 6027
  readonly name = "InvalidUserMetadataPDA"
  readonly msg = "[L127] Invalid User Metadata PDA"

  constructor(readonly logs?: string[]) {
    super("6027: [L127] Invalid User Metadata PDA")
  }
}

export class InvalidUserRewardsVaultPDA extends Error {
  static readonly code = 6028
  readonly code = 6028
  readonly name = "InvalidUserRewardsVaultPDA"
  readonly msg = "[L128] Invalid User Rewards Vault PDA"

  constructor(readonly logs?: string[]) {
    super("6028: [L128] Invalid User Rewards Vault PDA")
  }
}

export class InvalidLottoTicketPDA extends Error {
  static readonly code = 6029
  readonly code = 6029
  readonly name = "InvalidLottoTicketPDA"
  readonly msg = "[L129] Invalid Lotto Ticket PDA"

  constructor(readonly logs?: string[]) {
    super("6029: [L129] Invalid Lotto Ticket PDA")
  }
}

export class JackpotWinningNumbersNotUpdated extends Error {
  static readonly code = 6030
  readonly code = 6030
  readonly name = "JackpotWinningNumbersNotUpdated"
  readonly msg = "[L130] Jackpot Winning numbers not updated"

  constructor(readonly logs?: string[]) {
    super("6030: [L130] Jackpot Winning numbers not updated")
  }
}

export class JackpotAmountAlreadyDisbursed extends Error {
  static readonly code = 6031
  readonly code = 6031
  readonly name = "JackpotAmountAlreadyDisbursed"
  readonly msg = "[L131] Jackpot amount already disbursed"

  constructor(readonly logs?: string[]) {
    super("6031: [L131] Jackpot amount already disbursed")
  }
}

export class Tier1WinningNumbersNotUpdated extends Error {
  static readonly code = 6032
  readonly code = 6032
  readonly name = "Tier1WinningNumbersNotUpdated"
  readonly msg = "[L132] Tier 1 Winning number not updated"

  constructor(readonly logs?: string[]) {
    super("6032: [L132] Tier 1 Winning number not updated")
  }
}

export class Tier1AmountAlreadyDisbursed extends Error {
  static readonly code = 6033
  readonly code = 6033
  readonly name = "Tier1AmountAlreadyDisbursed"
  readonly msg = "[L133] Tier 1 amount already disbursed"

  constructor(readonly logs?: string[]) {
    super("6033: [L133] Tier 1 amount already disbursed")
  }
}

export class Tier2WinningNumbersNotUpdated extends Error {
  static readonly code = 6034
  readonly code = 6034
  readonly name = "Tier2WinningNumbersNotUpdated"
  readonly msg = "[L134] Tier 2 Winning number not updated"

  constructor(readonly logs?: string[]) {
    super("6034: [L134] Tier 2 Winning number not updated")
  }
}

export class Tier2AmountAlreadyDisbursed extends Error {
  static readonly code = 6035
  readonly code = 6035
  readonly name = "Tier2AmountAlreadyDisbursed"
  readonly msg = "[L135] Tier 2 amount already disbursed"

  constructor(readonly logs?: string[]) {
    super("6035: [L135] Tier 2 amount already disbursed")
  }
}

export class Tier3WinningNumbersNotUpdated extends Error {
  static readonly code = 6036
  readonly code = 6036
  readonly name = "Tier3WinningNumbersNotUpdated"
  readonly msg = "[L136] Tier 3 Winning number not updated"

  constructor(readonly logs?: string[]) {
    super("6036: [L136] Tier 3 Winning number not updated")
  }
}

export class Tier3AmountAlreadyDisbursed extends Error {
  static readonly code = 6037
  readonly code = 6037
  readonly name = "Tier3AmountAlreadyDisbursed"
  readonly msg = "[L137] Tier 3 amount already disbursed"

  constructor(readonly logs?: string[]) {
    super("6037: [L137] Tier 3 amount already disbursed")
  }
}

export class InvalidWinningTier extends Error {
  static readonly code = 6038
  readonly code = 6038
  readonly name = "InvalidWinningTier"
  readonly msg = "[L138] Invalid winning tier"

  constructor(readonly logs?: string[]) {
    super("6038: [L138] Invalid winning tier")
  }
}

export class NoDuplicateTicketsFound extends Error {
  static readonly code = 6039
  readonly code = 6039
  readonly name = "NoDuplicateTicketsFound"
  readonly msg = "[L139] No Duplicate tickets found"

  constructor(readonly logs?: string[]) {
    super("6039: [L139] No Duplicate tickets found")
  }
}

export class OnDemandRandomnessNotResolved extends Error {
  static readonly code = 6040
  readonly code = 6040
  readonly name = "OnDemandRandomnessNotResolved"
  readonly msg = "[L140] On Demad Randomness not resolved"

  constructor(readonly logs?: string[]) {
    super("6040: [L140] On Demad Randomness not resolved")
  }
}

export function fromCode(code: number, logs?: string[]): CustomError | null {
  switch (code) {
    case 6000:
      return new OverflowError(logs)
    case 6001:
      return new JupiterIxSourceTokenAccountMismatch(logs)
    case 6002:
      return new JupiterIxDestinationTokenAccountMismatch(logs)
    case 6003:
      return new OnlyLOLLYBuringAllowed(logs)
    case 6004:
      return new TokenAccountAuthorityMismatch(logs)
    case 6005:
      return new OnlySwapToLOLLYAllowed(logs)
    case 6006:
      return new OnlySwapFromUSDCAllowed(logs)
    case 6007:
      return new InsufficientFunds(logs)
    case 6008:
      return new LottoGameNotOpen(logs)
    case 6009:
      return new InvalidRound(logs)
    case 6010:
      return new InvalidWinningTicket(logs)
    case 6011:
      return new AlreadyDeclaredWinner(logs)
    case 6012:
      return new GameNotClosed(logs)
    case 6013:
      return new RoundNumbersAreSequential(logs)
    case 6014:
      return new LottoGameEnded(logs)
    case 6015:
      return new LottoGameIsStillOpen(logs)
    case 6016:
      return new NoRewardsToClaimFromVault(logs)
    case 6017:
      return new NotSufficientRewardsInVault(logs)
    case 6018:
      return new LottoGameVaultNotEmpty(logs)
    case 6019:
      return new InvalidNumbersInTicket(logs)
    case 6020:
      return new DuplicateWinningNumbers(logs)
    case 6021:
      return new WinningNumberIndexIsNotProvided(logs)
    case 6022:
      return new WinningNumbersNotSet(logs)
    case 6023:
      return new MathError(logs)
    case 6024:
      return new NoAvailableSlots(logs)
    case 6025:
      return new InvalidWinningNumberIndex(logs)
    case 6026:
      return new InvalidCrankAccounts(logs)
    case 6027:
      return new InvalidUserMetadataPDA(logs)
    case 6028:
      return new InvalidUserRewardsVaultPDA(logs)
    case 6029:
      return new InvalidLottoTicketPDA(logs)
    case 6030:
      return new JackpotWinningNumbersNotUpdated(logs)
    case 6031:
      return new JackpotAmountAlreadyDisbursed(logs)
    case 6032:
      return new Tier1WinningNumbersNotUpdated(logs)
    case 6033:
      return new Tier1AmountAlreadyDisbursed(logs)
    case 6034:
      return new Tier2WinningNumbersNotUpdated(logs)
    case 6035:
      return new Tier2AmountAlreadyDisbursed(logs)
    case 6036:
      return new Tier3WinningNumbersNotUpdated(logs)
    case 6037:
      return new Tier3AmountAlreadyDisbursed(logs)
    case 6038:
      return new InvalidWinningTier(logs)
    case 6039:
      return new NoDuplicateTicketsFound(logs)
    case 6040:
      return new OnDemandRandomnessNotResolved(logs)
  }

  return null
}
