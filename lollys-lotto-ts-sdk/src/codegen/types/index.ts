import * as RandomnessRequestError from "./RandomnessRequestError"
import * as ProgramInstruction from "./ProgramInstruction"
import * as LollysLottoProgramEventData from "./LollysLottoProgramEventData"
import * as LottoGameVersion from "./LottoGameVersion"
import * as LottoGameState from "./LottoGameState"
import * as WinningNumberUpdateState from "./WinningNumberUpdateState"
import * as WinningAmountDisbursedState from "./WinningAmountDisbursedState"
import * as UserTier from "./UserTier"

export { StartLottoGameParams } from "./StartLottoGameParams"
export type {
  StartLottoGameParamsFields,
  StartLottoGameParamsJSON,
} from "./StartLottoGameParams"
export { BurnLollyEvent } from "./BurnLollyEvent"
export type { BurnLollyEventFields, BurnLollyEventJSON } from "./BurnLollyEvent"
export { CloseEventEmitterEvent } from "./CloseEventEmitterEvent"
export type {
  CloseEventEmitterEventFields,
  CloseEventEmitterEventJSON,
} from "./CloseEventEmitterEvent"
export { CloseLollyBurnStateEvent } from "./CloseLollyBurnStateEvent"
export type {
  CloseLollyBurnStateEventFields,
  CloseLollyBurnStateEventJSON,
} from "./CloseLollyBurnStateEvent"
export { CloseLollysLottoEvent } from "./CloseLollysLottoEvent"
export type {
  CloseLollysLottoEventFields,
  CloseLollysLottoEventJSON,
} from "./CloseLollysLottoEvent"
export { CloseLottoGameEvent } from "./CloseLottoGameEvent"
export type {
  CloseLottoGameEventFields,
  CloseLottoGameEventJSON,
} from "./CloseLottoGameEvent"
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
export { CrankLottoGameClosedEvent } from "./CrankLottoGameClosedEvent"
export type {
  CrankLottoGameClosedEventFields,
  CrankLottoGameClosedEventJSON,
} from "./CrankLottoGameClosedEvent"
export { ProcessWinningNumbersEvent } from "./ProcessWinningNumbersEvent"
export type {
  ProcessWinningNumbersEventFields,
  ProcessWinningNumbersEventJSON,
} from "./ProcessWinningNumbersEvent"
export { DuplicateWinningNumbersEvent } from "./DuplicateWinningNumbersEvent"
export type {
  DuplicateWinningNumbersEventFields,
  DuplicateWinningNumbersEventJSON,
} from "./DuplicateWinningNumbersEvent"
export { RequestWinningNumbersEvent } from "./RequestWinningNumbersEvent"
export type {
  RequestWinningNumbersEventFields,
  RequestWinningNumbersEventJSON,
} from "./RequestWinningNumbersEvent"
export { TestEmitWinningNumbersEvent } from "./TestEmitWinningNumbersEvent"
export type {
  TestEmitWinningNumbersEventFields,
  TestEmitWinningNumbersEventJSON,
} from "./TestEmitWinningNumbersEvent"
export { BuyLottoTicketEvent } from "./BuyLottoTicketEvent"
export type {
  BuyLottoTicketEventFields,
  BuyLottoTicketEventJSON,
} from "./BuyLottoTicketEvent"
export { ClaimUserRewardsEvent } from "./ClaimUserRewardsEvent"
export type {
  ClaimUserRewardsEventFields,
  ClaimUserRewardsEventJSON,
} from "./ClaimUserRewardsEvent"
export { CloseLottoTicketEvent } from "./CloseLottoTicketEvent"
export type {
  CloseLottoTicketEventFields,
  CloseLottoTicketEventJSON,
} from "./CloseLottoTicketEvent"
export { CloseUserMetadataEvent } from "./CloseUserMetadataEvent"
export type {
  CloseUserMetadataEventFields,
  CloseUserMetadataEventJSON,
} from "./CloseUserMetadataEvent"
export { CreateUserMetadataEvent } from "./CreateUserMetadataEvent"
export type {
  CreateUserMetadataEventFields,
  CreateUserMetadataEventJSON,
} from "./CreateUserMetadataEvent"
export { CrankLottoGameWinnersEvent } from "./CrankLottoGameWinnersEvent"
export type {
  CrankLottoGameWinnersEventFields,
  CrankLottoGameWinnersEventJSON,
} from "./CrankLottoGameWinnersEvent"
export { CrankTransferWinningAmountToUserRewardsVaultEvent } from "./CrankTransferWinningAmountToUserRewardsVaultEvent"
export type {
  CrankTransferWinningAmountToUserRewardsVaultEventFields,
  CrankTransferWinningAmountToUserRewardsVaultEventJSON,
} from "./CrankTransferWinningAmountToUserRewardsVaultEvent"
export { CrankTransferToBuyAndBurnVaultEvent } from "./CrankTransferToBuyAndBurnVaultEvent"
export type {
  CrankTransferToBuyAndBurnVaultEventFields,
  CrankTransferToBuyAndBurnVaultEventJSON,
} from "./CrankTransferToBuyAndBurnVaultEvent"
export { LottoGameWinningNumbers } from "./LottoGameWinningNumbers"
export type {
  LottoGameWinningNumbersFields,
  LottoGameWinningNumbersJSON,
} from "./LottoGameWinningNumbers"
export { LottoTicketNumbers } from "./LottoTicketNumbers"
export type {
  LottoTicketNumbersFields,
  LottoTicketNumbersJSON,
} from "./LottoTicketNumbers"
export { RandomnessRequestError }

/** Custom error code: 7100 + idx => 0x17D4 + 0x${idx} */
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
  | ProgramInstruction.BurnLolly
  | ProgramInstruction.CloseEventEmitter
  | ProgramInstruction.CloseLollyBurnState
  | ProgramInstruction.CloseLollysLotto
  | ProgramInstruction.CloseLottoGame
  | ProgramInstruction.CreateLollyBurnState
  | ProgramInstruction.CreateLollysLotto
  | ProgramInstruction.StartLottoGame
  | ProgramInstruction.SwapUsdcLolly
  | ProgramInstruction.ProcessWinningNumbers
  | ProgramInstruction.RequestWinningNumbers
  | ProgramInstruction.TestEmitWinningNumbers
  | ProgramInstruction.BuyLottoTicket
  | ProgramInstruction.ClaimUserRewards
  | ProgramInstruction.CloseLottoTicket
  | ProgramInstruction.CloseUserMetadata
  | ProgramInstruction.CreateUserMetadata
  | ProgramInstruction.CrankLottoGameClosed
  | ProgramInstruction.CrankLottoGameWinners
  | ProgramInstruction.CrankTransferWinningAmountToUserRewardsVault
  | ProgramInstruction.CrankTransferToBuyAndBurnVault
export type ProgramInstructionJSON =
  | ProgramInstruction.BurnLollyJSON
  | ProgramInstruction.CloseEventEmitterJSON
  | ProgramInstruction.CloseLollyBurnStateJSON
  | ProgramInstruction.CloseLollysLottoJSON
  | ProgramInstruction.CloseLottoGameJSON
  | ProgramInstruction.CreateLollyBurnStateJSON
  | ProgramInstruction.CreateLollysLottoJSON
  | ProgramInstruction.StartLottoGameJSON
  | ProgramInstruction.SwapUsdcLollyJSON
  | ProgramInstruction.ProcessWinningNumbersJSON
  | ProgramInstruction.RequestWinningNumbersJSON
  | ProgramInstruction.TestEmitWinningNumbersJSON
  | ProgramInstruction.BuyLottoTicketJSON
  | ProgramInstruction.ClaimUserRewardsJSON
  | ProgramInstruction.CloseLottoTicketJSON
  | ProgramInstruction.CloseUserMetadataJSON
  | ProgramInstruction.CreateUserMetadataJSON
  | ProgramInstruction.CrankLottoGameClosedJSON
  | ProgramInstruction.CrankLottoGameWinnersJSON
  | ProgramInstruction.CrankTransferWinningAmountToUserRewardsVaultJSON
  | ProgramInstruction.CrankTransferToBuyAndBurnVaultJSON

export { LollysLottoProgramEventData }

/** The inner data of an [LollysLottoProgramEvent] */
export type LollysLottoProgramEventDataKind =
  | LollysLottoProgramEventData.BurnLolly
  | LollysLottoProgramEventData.CloseEventEmitter
  | LollysLottoProgramEventData.CloseLollyBurnState
  | LollysLottoProgramEventData.CloseLollysLotto
  | LollysLottoProgramEventData.CloseLottoGame
  | LollysLottoProgramEventData.CreateLollyBurnState
  | LollysLottoProgramEventData.CreateLollysLotto
  | LollysLottoProgramEventData.StartLottoGame
  | LollysLottoProgramEventData.SwapUsdcLolly
  | LollysLottoProgramEventData.ProcessWinningNumbers
  | LollysLottoProgramEventData.RequestWinningNumbers
  | LollysLottoProgramEventData.TestEmitWinningNumbers
  | LollysLottoProgramEventData.BuyLottoTicket
  | LollysLottoProgramEventData.ClaimUserRewards
  | LollysLottoProgramEventData.CloseLottoTicket
  | LollysLottoProgramEventData.CloseUserMetadata
  | LollysLottoProgramEventData.CreateUserMetadata
  | LollysLottoProgramEventData.CrankLottoGameClosed
  | LollysLottoProgramEventData.CrankLottoGameWinners
  | LollysLottoProgramEventData.CrankTransferWinningAmountToUserRewardsVault
  | LollysLottoProgramEventData.CrankTransferToBuyAndBurnVault
  | LollysLottoProgramEventData.DuplicateWinningNumbers
export type LollysLottoProgramEventDataJSON =
  | LollysLottoProgramEventData.BurnLollyJSON
  | LollysLottoProgramEventData.CloseEventEmitterJSON
  | LollysLottoProgramEventData.CloseLollyBurnStateJSON
  | LollysLottoProgramEventData.CloseLollysLottoJSON
  | LollysLottoProgramEventData.CloseLottoGameJSON
  | LollysLottoProgramEventData.CreateLollyBurnStateJSON
  | LollysLottoProgramEventData.CreateLollysLottoJSON
  | LollysLottoProgramEventData.StartLottoGameJSON
  | LollysLottoProgramEventData.SwapUsdcLollyJSON
  | LollysLottoProgramEventData.ProcessWinningNumbersJSON
  | LollysLottoProgramEventData.RequestWinningNumbersJSON
  | LollysLottoProgramEventData.TestEmitWinningNumbersJSON
  | LollysLottoProgramEventData.BuyLottoTicketJSON
  | LollysLottoProgramEventData.ClaimUserRewardsJSON
  | LollysLottoProgramEventData.CloseLottoTicketJSON
  | LollysLottoProgramEventData.CloseUserMetadataJSON
  | LollysLottoProgramEventData.CreateUserMetadataJSON
  | LollysLottoProgramEventData.CrankLottoGameClosedJSON
  | LollysLottoProgramEventData.CrankLottoGameWinnersJSON
  | LollysLottoProgramEventData.CrankTransferWinningAmountToUserRewardsVaultJSON
  | LollysLottoProgramEventData.CrankTransferToBuyAndBurnVaultJSON
  | LollysLottoProgramEventData.DuplicateWinningNumbersJSON

export { LottoGameVersion }

export type LottoGameVersionKind = LottoGameVersion.V1
export type LottoGameVersionJSON = LottoGameVersion.V1JSON

export { LottoGameState }

export type LottoGameStateKind =
  | LottoGameState.NotStarted
  | LottoGameState.Open
  | LottoGameState.Closed
  | LottoGameState.Finished
export type LottoGameStateJSON =
  | LottoGameState.NotStartedJSON
  | LottoGameState.OpenJSON
  | LottoGameState.ClosedJSON
  | LottoGameState.FinishedJSON

export { WinningNumberUpdateState }

export type WinningNumberUpdateStateKind =
  | WinningNumberUpdateState.NotUpdated
  | WinningNumberUpdateState.Updated
export type WinningNumberUpdateStateJSON =
  | WinningNumberUpdateState.NotUpdatedJSON
  | WinningNumberUpdateState.UpdatedJSON

export { WinningAmountDisbursedState }

export type WinningAmountDisbursedStateKind =
  | WinningAmountDisbursedState.NotDisbursed
  | WinningAmountDisbursedState.Disbursed
export type WinningAmountDisbursedStateJSON =
  | WinningAmountDisbursedState.NotDisbursedJSON
  | WinningAmountDisbursedState.DisbursedJSON

export { UserTier }

export type UserTierKind = UserTier.Bronze
export type UserTierJSON = UserTier.BronzeJSON
