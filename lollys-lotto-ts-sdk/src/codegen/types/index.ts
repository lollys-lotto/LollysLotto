import * as RandomnessRequestError from "./RandomnessRequestError"
import * as ProgramInstruction from "./ProgramInstruction"
import * as LollysLottoEventData from "./LollysLottoEventData"
import * as LottoGameState from "./LottoGameState"
import * as UserTier from "./UserTier"

export { StartLottoGameParams } from "./StartLottoGameParams"
export type {
  StartLottoGameParamsFields,
  StartLottoGameParamsJSON,
} from "./StartLottoGameParams"
export { ProcessWinningNumbersEvent } from "./ProcessWinningNumbersEvent"
export type {
  ProcessWinningNumbersEventFields,
  ProcessWinningNumbersEventJSON,
} from "./ProcessWinningNumbersEvent"
export { RequestWinningNumbersEvent } from "./RequestWinningNumbersEvent"
export type {
  RequestWinningNumbersEventFields,
  RequestWinningNumbersEventJSON,
} from "./RequestWinningNumbersEvent"
export { BurnLollyEvent } from "./BurnLollyEvent"
export type { BurnLollyEventFields, BurnLollyEventJSON } from "./BurnLollyEvent"
export { BuyLottoTicketEvent } from "./BuyLottoTicketEvent"
export type {
  BuyLottoTicketEventFields,
  BuyLottoTicketEventJSON,
} from "./BuyLottoTicketEvent"
export { CrankLottoGameWinnerEvent } from "./CrankLottoGameWinnerEvent"
export type {
  CrankLottoGameWinnerEventFields,
  CrankLottoGameWinnerEventJSON,
} from "./CrankLottoGameWinnerEvent"
export { CreateLollyBurnStateEvent } from "./CreateLollyBurnStateEvent"
export type {
  CreateLollyBurnStateEventFields,
  CreateLollyBurnStateEventJSON,
} from "./CreateLollyBurnStateEvent"
export { CreateLollysLottoEvent } from "./CreateLollysLottoEvent"
export type {
  CreateLollysLottoEventFields,
  CreateLollysLottoEventJSON,
} from "./CreateLollysLottoEvent"
export { CreateUserMetadataEvent } from "./CreateUserMetadataEvent"
export type {
  CreateUserMetadataEventFields,
  CreateUserMetadataEventJSON,
} from "./CreateUserMetadataEvent"
export { StartLottoGameEvent } from "./StartLottoGameEvent"
export type {
  StartLottoGameEventFields,
  StartLottoGameEventJSON,
} from "./StartLottoGameEvent"
export { SwapUsdcLollyEvent } from "./SwapUsdcLollyEvent"
export type {
  SwapUsdcLollyEventFields,
  SwapUsdcLollyEventJSON,
} from "./SwapUsdcLollyEvent"
export { ClaimTicket } from "./ClaimTicket"
export type { ClaimTicketFields, ClaimTicketJSON } from "./ClaimTicket"
export { RandomnessRequestError }

export type RandomnessRequestErrorKind =
  | RandomnessRequestError.InvalidAuthority
  | RandomnessRequestError.InvalidEscrow
  | RandomnessRequestError.ArrayOverflow
  | RandomnessRequestError.StaleData
  | RandomnessRequestError.InvalidTrustedSigner
  | RandomnessRequestError.InvalidMrEnclave
  | RandomnessRequestError.InvalidSymbol
  | RandomnessRequestError.IncorrectSwitchboardFunction
  | RandomnessRequestError.InvalidSwitchboardFunction
  | RandomnessRequestError.FunctionValidationFailed
  | RandomnessRequestError.SwitchboardRequestNotSuccessful
  | RandomnessRequestError.RoundInactive
  | RandomnessRequestError.HouseInsufficientFunds
export type RandomnessRequestErrorJSON =
  | RandomnessRequestError.InvalidAuthorityJSON
  | RandomnessRequestError.InvalidEscrowJSON
  | RandomnessRequestError.ArrayOverflowJSON
  | RandomnessRequestError.StaleDataJSON
  | RandomnessRequestError.InvalidTrustedSignerJSON
  | RandomnessRequestError.InvalidMrEnclaveJSON
  | RandomnessRequestError.InvalidSymbolJSON
  | RandomnessRequestError.IncorrectSwitchboardFunctionJSON
  | RandomnessRequestError.InvalidSwitchboardFunctionJSON
  | RandomnessRequestError.FunctionValidationFailedJSON
  | RandomnessRequestError.SwitchboardRequestNotSuccessfulJSON
  | RandomnessRequestError.RoundInactiveJSON
  | RandomnessRequestError.HouseInsufficientFundsJSON

export { ProgramInstruction }

/**
 * Used to decode the type of instruction that occurred, you can decode
 * this from a historical transaction's raw instruction data.
 */
export type ProgramInstructionKind =
  | ProgramInstruction.ProcessWinningNumbers
  | ProgramInstruction.RequestWinningNumbers
  | ProgramInstruction.BurnLolly
  | ProgramInstruction.BuyLottoTicket
  | ProgramInstruction.CrankLottoGameWinner
  | ProgramInstruction.CreateLollyBurnState
  | ProgramInstruction.CreateLollysLotto
  | ProgramInstruction.CreateUserMetadata
  | ProgramInstruction.StartLottoGame
  | ProgramInstruction.SwapUsdcLolly
export type ProgramInstructionJSON =
  | ProgramInstruction.ProcessWinningNumbersJSON
  | ProgramInstruction.RequestWinningNumbersJSON
  | ProgramInstruction.BurnLollyJSON
  | ProgramInstruction.BuyLottoTicketJSON
  | ProgramInstruction.CrankLottoGameWinnerJSON
  | ProgramInstruction.CreateLollyBurnStateJSON
  | ProgramInstruction.CreateLollysLottoJSON
  | ProgramInstruction.CreateUserMetadataJSON
  | ProgramInstruction.StartLottoGameJSON
  | ProgramInstruction.SwapUsdcLollyJSON

export { LollysLottoEventData }

/** The inner data of an [LollysLottoEvent] */
export type LollysLottoEventDataKind =
  | LollysLottoEventData.ProcessWinningNumbers
  | LollysLottoEventData.RequestWinningNumbers
  | LollysLottoEventData.BurnLolly
  | LollysLottoEventData.BuyLottoTicket
  | LollysLottoEventData.CrankLottoGameWinner
  | LollysLottoEventData.CreateLollyBurnState
  | LollysLottoEventData.CreateLollysLotto
  | LollysLottoEventData.CreateUserMetadata
  | LollysLottoEventData.StartLottoGame
  | LollysLottoEventData.SwapUsdcLolly
export type LollysLottoEventDataJSON =
  | LollysLottoEventData.ProcessWinningNumbersJSON
  | LollysLottoEventData.RequestWinningNumbersJSON
  | LollysLottoEventData.BurnLollyJSON
  | LollysLottoEventData.BuyLottoTicketJSON
  | LollysLottoEventData.CrankLottoGameWinnerJSON
  | LollysLottoEventData.CreateLollyBurnStateJSON
  | LollysLottoEventData.CreateLollysLottoJSON
  | LollysLottoEventData.CreateUserMetadataJSON
  | LollysLottoEventData.StartLottoGameJSON
  | LollysLottoEventData.SwapUsdcLollyJSON

export { LottoGameState }

export type LottoGameStateKind =
  | LottoGameState.Open
  | LottoGameState.Closed
  | LottoGameState.Finished
export type LottoGameStateJSON =
  | LottoGameState.OpenJSON
  | LottoGameState.ClosedJSON
  | LottoGameState.FinishedJSON

export { UserTier }

export type UserTierKind = UserTier.Bronze
export type UserTierJSON = UserTier.BronzeJSON
