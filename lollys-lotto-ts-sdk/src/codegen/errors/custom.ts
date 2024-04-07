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
  | GameNotFinished

export class OverflowError extends Error {
  static readonly code = 6000
  readonly code = 6000
  readonly name = "OverflowError"
  readonly msg = "Overflow"

  constructor(readonly logs?: string[]) {
    super("6000: Overflow")
  }
}

export class JupiterIxSourceTokenAccountMismatch extends Error {
  static readonly code = 6001
  readonly code = 6001
  readonly name = "JupiterIxSourceTokenAccountMismatch"
  readonly msg = "Source token account of jupiter ix mismatch"

  constructor(readonly logs?: string[]) {
    super("6001: Source token account of jupiter ix mismatch")
  }
}

export class JupiterIxDestinationTokenAccountMismatch extends Error {
  static readonly code = 6002
  readonly code = 6002
  readonly name = "JupiterIxDestinationTokenAccountMismatch"
  readonly msg = "Destination token account of jupiter ix mismatch"

  constructor(readonly logs?: string[]) {
    super("6002: Destination token account of jupiter ix mismatch")
  }
}

export class OnlyLOLLYBuringAllowed extends Error {
  static readonly code = 6003
  readonly code = 6003
  readonly name = "OnlyLOLLYBuringAllowed"
  readonly msg = "Only $LOLLY tokens are allowed to burn!"

  constructor(readonly logs?: string[]) {
    super("6003: Only $LOLLY tokens are allowed to burn!")
  }
}

export class TokenAccountAuthorityMismatch extends Error {
  static readonly code = 6004
  readonly code = 6004
  readonly name = "TokenAccountAuthorityMismatch"
  readonly msg = "Token Account authority mismatch!"

  constructor(readonly logs?: string[]) {
    super("6004: Token Account authority mismatch!")
  }
}

export class OnlySwapToLOLLYAllowed extends Error {
  static readonly code = 6005
  readonly code = 6005
  readonly name = "OnlySwapToLOLLYAllowed"
  readonly msg = "Only Swap to $LOLLY tokens are allowed"

  constructor(readonly logs?: string[]) {
    super("6005: Only Swap to $LOLLY tokens are allowed")
  }
}

export class OnlySwapFromUSDCAllowed extends Error {
  static readonly code = 6006
  readonly code = 6006
  readonly name = "OnlySwapFromUSDCAllowed"
  readonly msg = "Only Swap from $USDC tokens are allowed"

  constructor(readonly logs?: string[]) {
    super("6006: Only Swap from $USDC tokens are allowed")
  }
}

export class InsufficientFunds extends Error {
  static readonly code = 6007
  readonly code = 6007
  readonly name = "InsufficientFunds"
  readonly msg = "Insufficient funds"

  constructor(readonly logs?: string[]) {
    super("6007: Insufficient funds")
  }
}

export class LottoGameNotOpen extends Error {
  static readonly code = 6008
  readonly code = 6008
  readonly name = "LottoGameNotOpen"
  readonly msg = "Lotto Game not open"

  constructor(readonly logs?: string[]) {
    super("6008: Lotto Game not open")
  }
}

export class InvalidRound extends Error {
  static readonly code = 6009
  readonly code = 6009
  readonly name = "InvalidRound"
  readonly msg = "Invalid round"

  constructor(readonly logs?: string[]) {
    super("6009: Invalid round")
  }
}

export class InvalidWinningTicket extends Error {
  static readonly code = 6010
  readonly code = 6010
  readonly name = "InvalidWinningTicket"
  readonly msg = "Invalid winning ticket"

  constructor(readonly logs?: string[]) {
    super("6010: Invalid winning ticket")
  }
}

export class AlreadyDeclaredWinner extends Error {
  static readonly code = 6011
  readonly code = 6011
  readonly name = "AlreadyDeclaredWinner"
  readonly msg = "Ticket is already declared winner"

  constructor(readonly logs?: string[]) {
    super("6011: Ticket is already declared winner")
  }
}

export class GameNotClosed extends Error {
  static readonly code = 6012
  readonly code = 6012
  readonly name = "GameNotClosed"
  readonly msg = "Game not closed"

  constructor(readonly logs?: string[]) {
    super("6012: Game not closed")
  }
}

export class RoundNumbersAreSequential extends Error {
  static readonly code = 6013
  readonly code = 6013
  readonly name = "RoundNumbersAreSequential"
  readonly msg = "Round numbers have to be sequential"

  constructor(readonly logs?: string[]) {
    super("6013: Round numbers have to be sequential")
  }
}

export class LottoGameEnded extends Error {
  static readonly code = 6014
  readonly code = 6014
  readonly name = "LottoGameEnded"
  readonly msg = "Lotto Game has ended"

  constructor(readonly logs?: string[]) {
    super("6014: Lotto Game has ended")
  }
}

export class GameNotFinished extends Error {
  static readonly code = 6015
  readonly code = 6015
  readonly name = "GameNotFinished"
  readonly msg = "Game not finished"

  constructor(readonly logs?: string[]) {
    super("6015: Game not finished")
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
      return new GameNotFinished(logs)
  }

  return null
}
