import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export type BurnLollyFields = [types.BurnLollyEventFields]
export type BurnLollyValue = [types.BurnLollyEvent]

export interface BurnLollyJSON {
  kind: "BurnLolly"
  value: [types.BurnLollyEventJSON]
}

export class BurnLolly {
  static readonly discriminator = 0
  static readonly kind = "BurnLolly"
  readonly discriminator = 0
  readonly kind = "BurnLolly"
  readonly value: BurnLollyValue

  constructor(value: BurnLollyFields) {
    this.value = [new types.BurnLollyEvent({ ...value[0] })]
  }

  toJSON(): BurnLollyJSON {
    return {
      kind: "BurnLolly",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      BurnLolly: {
        _0: types.BurnLollyEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CloseEventEmitterFields = [types.CloseEventEmitterEventFields]
export type CloseEventEmitterValue = [types.CloseEventEmitterEvent]

export interface CloseEventEmitterJSON {
  kind: "CloseEventEmitter"
  value: [types.CloseEventEmitterEventJSON]
}

export class CloseEventEmitter {
  static readonly discriminator = 1
  static readonly kind = "CloseEventEmitter"
  readonly discriminator = 1
  readonly kind = "CloseEventEmitter"
  readonly value: CloseEventEmitterValue

  constructor(value: CloseEventEmitterFields) {
    this.value = [new types.CloseEventEmitterEvent({ ...value[0] })]
  }

  toJSON(): CloseEventEmitterJSON {
    return {
      kind: "CloseEventEmitter",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CloseEventEmitter: {
        _0: types.CloseEventEmitterEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CloseLollyBurnStateFields = [types.CloseLollyBurnStateEventFields]
export type CloseLollyBurnStateValue = [types.CloseLollyBurnStateEvent]

export interface CloseLollyBurnStateJSON {
  kind: "CloseLollyBurnState"
  value: [types.CloseLollyBurnStateEventJSON]
}

export class CloseLollyBurnState {
  static readonly discriminator = 2
  static readonly kind = "CloseLollyBurnState"
  readonly discriminator = 2
  readonly kind = "CloseLollyBurnState"
  readonly value: CloseLollyBurnStateValue

  constructor(value: CloseLollyBurnStateFields) {
    this.value = [new types.CloseLollyBurnStateEvent({ ...value[0] })]
  }

  toJSON(): CloseLollyBurnStateJSON {
    return {
      kind: "CloseLollyBurnState",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CloseLollyBurnState: {
        _0: types.CloseLollyBurnStateEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CloseLollysLottoFields = [types.CloseLollysLottoEventFields]
export type CloseLollysLottoValue = [types.CloseLollysLottoEvent]

export interface CloseLollysLottoJSON {
  kind: "CloseLollysLotto"
  value: [types.CloseLollysLottoEventJSON]
}

export class CloseLollysLotto {
  static readonly discriminator = 3
  static readonly kind = "CloseLollysLotto"
  readonly discriminator = 3
  readonly kind = "CloseLollysLotto"
  readonly value: CloseLollysLottoValue

  constructor(value: CloseLollysLottoFields) {
    this.value = [new types.CloseLollysLottoEvent({ ...value[0] })]
  }

  toJSON(): CloseLollysLottoJSON {
    return {
      kind: "CloseLollysLotto",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CloseLollysLotto: {
        _0: types.CloseLollysLottoEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CloseLottoGameFields = [types.CloseLottoGameEventFields]
export type CloseLottoGameValue = [types.CloseLottoGameEvent]

export interface CloseLottoGameJSON {
  kind: "CloseLottoGame"
  value: [types.CloseLottoGameEventJSON]
}

export class CloseLottoGame {
  static readonly discriminator = 4
  static readonly kind = "CloseLottoGame"
  readonly discriminator = 4
  readonly kind = "CloseLottoGame"
  readonly value: CloseLottoGameValue

  constructor(value: CloseLottoGameFields) {
    this.value = [new types.CloseLottoGameEvent({ ...value[0] })]
  }

  toJSON(): CloseLottoGameJSON {
    return {
      kind: "CloseLottoGame",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CloseLottoGame: {
        _0: types.CloseLottoGameEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CreateLollyBurnStateFields = [types.CreateLollyBurnStateEventFields]
export type CreateLollyBurnStateValue = [types.CreateLollyBurnStateEvent]

export interface CreateLollyBurnStateJSON {
  kind: "CreateLollyBurnState"
  value: [types.CreateLollyBurnStateEventJSON]
}

export class CreateLollyBurnState {
  static readonly discriminator = 5
  static readonly kind = "CreateLollyBurnState"
  readonly discriminator = 5
  readonly kind = "CreateLollyBurnState"
  readonly value: CreateLollyBurnStateValue

  constructor(value: CreateLollyBurnStateFields) {
    this.value = [new types.CreateLollyBurnStateEvent({ ...value[0] })]
  }

  toJSON(): CreateLollyBurnStateJSON {
    return {
      kind: "CreateLollyBurnState",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CreateLollyBurnState: {
        _0: types.CreateLollyBurnStateEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CreateLollysLottoFields = [types.CreateLollysLottoEventFields]
export type CreateLollysLottoValue = [types.CreateLollysLottoEvent]

export interface CreateLollysLottoJSON {
  kind: "CreateLollysLotto"
  value: [types.CreateLollysLottoEventJSON]
}

export class CreateLollysLotto {
  static readonly discriminator = 6
  static readonly kind = "CreateLollysLotto"
  readonly discriminator = 6
  readonly kind = "CreateLollysLotto"
  readonly value: CreateLollysLottoValue

  constructor(value: CreateLollysLottoFields) {
    this.value = [new types.CreateLollysLottoEvent({ ...value[0] })]
  }

  toJSON(): CreateLollysLottoJSON {
    return {
      kind: "CreateLollysLotto",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CreateLollysLotto: {
        _0: types.CreateLollysLottoEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type StartLottoGameFields = [types.StartLottoGameEventFields]
export type StartLottoGameValue = [types.StartLottoGameEvent]

export interface StartLottoGameJSON {
  kind: "StartLottoGame"
  value: [types.StartLottoGameEventJSON]
}

export class StartLottoGame {
  static readonly discriminator = 7
  static readonly kind = "StartLottoGame"
  readonly discriminator = 7
  readonly kind = "StartLottoGame"
  readonly value: StartLottoGameValue

  constructor(value: StartLottoGameFields) {
    this.value = [new types.StartLottoGameEvent({ ...value[0] })]
  }

  toJSON(): StartLottoGameJSON {
    return {
      kind: "StartLottoGame",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      StartLottoGame: {
        _0: types.StartLottoGameEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type SwapUsdcLollyFields = [types.SwapUsdcLollyEventFields]
export type SwapUsdcLollyValue = [types.SwapUsdcLollyEvent]

export interface SwapUsdcLollyJSON {
  kind: "SwapUsdcLolly"
  value: [types.SwapUsdcLollyEventJSON]
}

export class SwapUsdcLolly {
  static readonly discriminator = 8
  static readonly kind = "SwapUsdcLolly"
  readonly discriminator = 8
  readonly kind = "SwapUsdcLolly"
  readonly value: SwapUsdcLollyValue

  constructor(value: SwapUsdcLollyFields) {
    this.value = [new types.SwapUsdcLollyEvent({ ...value[0] })]
  }

  toJSON(): SwapUsdcLollyJSON {
    return {
      kind: "SwapUsdcLolly",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      SwapUsdcLolly: {
        _0: types.SwapUsdcLollyEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type ProcessWinningNumbersFields = [
  types.ProcessWinningNumbersEventFields
]
export type ProcessWinningNumbersValue = [types.ProcessWinningNumbersEvent]

export interface ProcessWinningNumbersJSON {
  kind: "ProcessWinningNumbers"
  value: [types.ProcessWinningNumbersEventJSON]
}

export class ProcessWinningNumbers {
  static readonly discriminator = 9
  static readonly kind = "ProcessWinningNumbers"
  readonly discriminator = 9
  readonly kind = "ProcessWinningNumbers"
  readonly value: ProcessWinningNumbersValue

  constructor(value: ProcessWinningNumbersFields) {
    this.value = [new types.ProcessWinningNumbersEvent({ ...value[0] })]
  }

  toJSON(): ProcessWinningNumbersJSON {
    return {
      kind: "ProcessWinningNumbers",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      ProcessWinningNumbers: {
        _0: types.ProcessWinningNumbersEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type RequestWinningNumbersFields = [
  types.RequestWinningNumbersEventFields
]
export type RequestWinningNumbersValue = [types.RequestWinningNumbersEvent]

export interface RequestWinningNumbersJSON {
  kind: "RequestWinningNumbers"
  value: [types.RequestWinningNumbersEventJSON]
}

export class RequestWinningNumbers {
  static readonly discriminator = 10
  static readonly kind = "RequestWinningNumbers"
  readonly discriminator = 10
  readonly kind = "RequestWinningNumbers"
  readonly value: RequestWinningNumbersValue

  constructor(value: RequestWinningNumbersFields) {
    this.value = [new types.RequestWinningNumbersEvent({ ...value[0] })]
  }

  toJSON(): RequestWinningNumbersJSON {
    return {
      kind: "RequestWinningNumbers",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      RequestWinningNumbers: {
        _0: types.RequestWinningNumbersEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type TestEmitWinningNumbersFields = [
  types.TestEmitWinningNumbersEventFields
]
export type TestEmitWinningNumbersValue = [types.TestEmitWinningNumbersEvent]

export interface TestEmitWinningNumbersJSON {
  kind: "TestEmitWinningNumbers"
  value: [types.TestEmitWinningNumbersEventJSON]
}

export class TestEmitWinningNumbers {
  static readonly discriminator = 11
  static readonly kind = "TestEmitWinningNumbers"
  readonly discriminator = 11
  readonly kind = "TestEmitWinningNumbers"
  readonly value: TestEmitWinningNumbersValue

  constructor(value: TestEmitWinningNumbersFields) {
    this.value = [new types.TestEmitWinningNumbersEvent({ ...value[0] })]
  }

  toJSON(): TestEmitWinningNumbersJSON {
    return {
      kind: "TestEmitWinningNumbers",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      TestEmitWinningNumbers: {
        _0: types.TestEmitWinningNumbersEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type BuyLottoTicketFields = [types.BuyLottoTicketEventFields]
export type BuyLottoTicketValue = [types.BuyLottoTicketEvent]

export interface BuyLottoTicketJSON {
  kind: "BuyLottoTicket"
  value: [types.BuyLottoTicketEventJSON]
}

export class BuyLottoTicket {
  static readonly discriminator = 12
  static readonly kind = "BuyLottoTicket"
  readonly discriminator = 12
  readonly kind = "BuyLottoTicket"
  readonly value: BuyLottoTicketValue

  constructor(value: BuyLottoTicketFields) {
    this.value = [new types.BuyLottoTicketEvent({ ...value[0] })]
  }

  toJSON(): BuyLottoTicketJSON {
    return {
      kind: "BuyLottoTicket",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      BuyLottoTicket: {
        _0: types.BuyLottoTicketEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type ClaimUserRewardsFields = [types.ClaimUserRewardsEventFields]
export type ClaimUserRewardsValue = [types.ClaimUserRewardsEvent]

export interface ClaimUserRewardsJSON {
  kind: "ClaimUserRewards"
  value: [types.ClaimUserRewardsEventJSON]
}

export class ClaimUserRewards {
  static readonly discriminator = 13
  static readonly kind = "ClaimUserRewards"
  readonly discriminator = 13
  readonly kind = "ClaimUserRewards"
  readonly value: ClaimUserRewardsValue

  constructor(value: ClaimUserRewardsFields) {
    this.value = [new types.ClaimUserRewardsEvent({ ...value[0] })]
  }

  toJSON(): ClaimUserRewardsJSON {
    return {
      kind: "ClaimUserRewards",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      ClaimUserRewards: {
        _0: types.ClaimUserRewardsEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CloseLottoTicketFields = [types.CloseLottoTicketEventFields]
export type CloseLottoTicketValue = [types.CloseLottoTicketEvent]

export interface CloseLottoTicketJSON {
  kind: "CloseLottoTicket"
  value: [types.CloseLottoTicketEventJSON]
}

export class CloseLottoTicket {
  static readonly discriminator = 14
  static readonly kind = "CloseLottoTicket"
  readonly discriminator = 14
  readonly kind = "CloseLottoTicket"
  readonly value: CloseLottoTicketValue

  constructor(value: CloseLottoTicketFields) {
    this.value = [new types.CloseLottoTicketEvent({ ...value[0] })]
  }

  toJSON(): CloseLottoTicketJSON {
    return {
      kind: "CloseLottoTicket",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CloseLottoTicket: {
        _0: types.CloseLottoTicketEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CloseUserMetadataFields = [types.CloseUserMetadataEventFields]
export type CloseUserMetadataValue = [types.CloseUserMetadataEvent]

export interface CloseUserMetadataJSON {
  kind: "CloseUserMetadata"
  value: [types.CloseUserMetadataEventJSON]
}

export class CloseUserMetadata {
  static readonly discriminator = 15
  static readonly kind = "CloseUserMetadata"
  readonly discriminator = 15
  readonly kind = "CloseUserMetadata"
  readonly value: CloseUserMetadataValue

  constructor(value: CloseUserMetadataFields) {
    this.value = [new types.CloseUserMetadataEvent({ ...value[0] })]
  }

  toJSON(): CloseUserMetadataJSON {
    return {
      kind: "CloseUserMetadata",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CloseUserMetadata: {
        _0: types.CloseUserMetadataEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CreateUserMetadataFields = [types.CreateUserMetadataEventFields]
export type CreateUserMetadataValue = [types.CreateUserMetadataEvent]

export interface CreateUserMetadataJSON {
  kind: "CreateUserMetadata"
  value: [types.CreateUserMetadataEventJSON]
}

export class CreateUserMetadata {
  static readonly discriminator = 16
  static readonly kind = "CreateUserMetadata"
  readonly discriminator = 16
  readonly kind = "CreateUserMetadata"
  readonly value: CreateUserMetadataValue

  constructor(value: CreateUserMetadataFields) {
    this.value = [new types.CreateUserMetadataEvent({ ...value[0] })]
  }

  toJSON(): CreateUserMetadataJSON {
    return {
      kind: "CreateUserMetadata",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CreateUserMetadata: {
        _0: types.CreateUserMetadataEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CrankLottoGameClosedFields = [types.CrankLottoGameClosedEventFields]
export type CrankLottoGameClosedValue = [types.CrankLottoGameClosedEvent]

export interface CrankLottoGameClosedJSON {
  kind: "CrankLottoGameClosed"
  value: [types.CrankLottoGameClosedEventJSON]
}

export class CrankLottoGameClosed {
  static readonly discriminator = 17
  static readonly kind = "CrankLottoGameClosed"
  readonly discriminator = 17
  readonly kind = "CrankLottoGameClosed"
  readonly value: CrankLottoGameClosedValue

  constructor(value: CrankLottoGameClosedFields) {
    this.value = [new types.CrankLottoGameClosedEvent({ ...value[0] })]
  }

  toJSON(): CrankLottoGameClosedJSON {
    return {
      kind: "CrankLottoGameClosed",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CrankLottoGameClosed: {
        _0: types.CrankLottoGameClosedEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CrankLottoGameWinnersFields = [
  types.CrankLottoGameWinnersEventFields
]
export type CrankLottoGameWinnersValue = [types.CrankLottoGameWinnersEvent]

export interface CrankLottoGameWinnersJSON {
  kind: "CrankLottoGameWinners"
  value: [types.CrankLottoGameWinnersEventJSON]
}

export class CrankLottoGameWinners {
  static readonly discriminator = 18
  static readonly kind = "CrankLottoGameWinners"
  readonly discriminator = 18
  readonly kind = "CrankLottoGameWinners"
  readonly value: CrankLottoGameWinnersValue

  constructor(value: CrankLottoGameWinnersFields) {
    this.value = [new types.CrankLottoGameWinnersEvent({ ...value[0] })]
  }

  toJSON(): CrankLottoGameWinnersJSON {
    return {
      kind: "CrankLottoGameWinners",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CrankLottoGameWinners: {
        _0: types.CrankLottoGameWinnersEvent.toEncodable(this.value[0]),
      },
    }
  }
}

export type CrankTransferWinningAmountToUserRewardsVaultFields = [
  types.CrankTransferWinningAmountToUserRewardsVaultEventFields
]
export type CrankTransferWinningAmountToUserRewardsVaultValue = [
  types.CrankTransferWinningAmountToUserRewardsVaultEvent
]

export interface CrankTransferWinningAmountToUserRewardsVaultJSON {
  kind: "CrankTransferWinningAmountToUserRewardsVault"
  value: [types.CrankTransferWinningAmountToUserRewardsVaultEventJSON]
}

export class CrankTransferWinningAmountToUserRewardsVault {
  static readonly discriminator = 19
  static readonly kind = "CrankTransferWinningAmountToUserRewardsVault"
  readonly discriminator = 19
  readonly kind = "CrankTransferWinningAmountToUserRewardsVault"
  readonly value: CrankTransferWinningAmountToUserRewardsVaultValue

  constructor(value: CrankTransferWinningAmountToUserRewardsVaultFields) {
    this.value = [
      new types.CrankTransferWinningAmountToUserRewardsVaultEvent({
        ...value[0],
      }),
    ]
  }

  toJSON(): CrankTransferWinningAmountToUserRewardsVaultJSON {
    return {
      kind: "CrankTransferWinningAmountToUserRewardsVault",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CrankTransferWinningAmountToUserRewardsVault: {
        _0: types.CrankTransferWinningAmountToUserRewardsVaultEvent.toEncodable(
          this.value[0]
        ),
      },
    }
  }
}

export type CrankTransferToBuyAndBurnVaultFields = [
  types.CrankTransferToBuyAndBurnVaultEventFields
]
export type CrankTransferToBuyAndBurnVaultValue = [
  types.CrankTransferToBuyAndBurnVaultEvent
]

export interface CrankTransferToBuyAndBurnVaultJSON {
  kind: "CrankTransferToBuyAndBurnVault"
  value: [types.CrankTransferToBuyAndBurnVaultEventJSON]
}

export class CrankTransferToBuyAndBurnVault {
  static readonly discriminator = 20
  static readonly kind = "CrankTransferToBuyAndBurnVault"
  readonly discriminator = 20
  readonly kind = "CrankTransferToBuyAndBurnVault"
  readonly value: CrankTransferToBuyAndBurnVaultValue

  constructor(value: CrankTransferToBuyAndBurnVaultFields) {
    this.value = [
      new types.CrankTransferToBuyAndBurnVaultEvent({ ...value[0] }),
    ]
  }

  toJSON(): CrankTransferToBuyAndBurnVaultJSON {
    return {
      kind: "CrankTransferToBuyAndBurnVault",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CrankTransferToBuyAndBurnVault: {
        _0: types.CrankTransferToBuyAndBurnVaultEvent.toEncodable(
          this.value[0]
        ),
      },
    }
  }
}

export type DuplicateWinningNumbersFields = [
  types.DuplicateWinningNumbersEventFields
]
export type DuplicateWinningNumbersValue = [types.DuplicateWinningNumbersEvent]

export interface DuplicateWinningNumbersJSON {
  kind: "DuplicateWinningNumbers"
  value: [types.DuplicateWinningNumbersEventJSON]
}

export class DuplicateWinningNumbers {
  static readonly discriminator = 21
  static readonly kind = "DuplicateWinningNumbers"
  readonly discriminator = 21
  readonly kind = "DuplicateWinningNumbers"
  readonly value: DuplicateWinningNumbersValue

  constructor(value: DuplicateWinningNumbersFields) {
    this.value = [new types.DuplicateWinningNumbersEvent({ ...value[0] })]
  }

  toJSON(): DuplicateWinningNumbersJSON {
    return {
      kind: "DuplicateWinningNumbers",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      DuplicateWinningNumbers: {
        _0: types.DuplicateWinningNumbersEvent.toEncodable(this.value[0]),
      },
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.LollysLottoProgramEventDataKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("BurnLolly" in obj) {
    const val = obj["BurnLolly"]
    return new BurnLolly([types.BurnLollyEvent.fromDecoded(val["_0"])])
  }
  if ("CloseEventEmitter" in obj) {
    const val = obj["CloseEventEmitter"]
    return new CloseEventEmitter([
      types.CloseEventEmitterEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CloseLollyBurnState" in obj) {
    const val = obj["CloseLollyBurnState"]
    return new CloseLollyBurnState([
      types.CloseLollyBurnStateEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CloseLollysLotto" in obj) {
    const val = obj["CloseLollysLotto"]
    return new CloseLollysLotto([
      types.CloseLollysLottoEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CloseLottoGame" in obj) {
    const val = obj["CloseLottoGame"]
    return new CloseLottoGame([
      types.CloseLottoGameEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CreateLollyBurnState" in obj) {
    const val = obj["CreateLollyBurnState"]
    return new CreateLollyBurnState([
      types.CreateLollyBurnStateEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CreateLollysLotto" in obj) {
    const val = obj["CreateLollysLotto"]
    return new CreateLollysLotto([
      types.CreateLollysLottoEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("StartLottoGame" in obj) {
    const val = obj["StartLottoGame"]
    return new StartLottoGame([
      types.StartLottoGameEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("SwapUsdcLolly" in obj) {
    const val = obj["SwapUsdcLolly"]
    return new SwapUsdcLolly([types.SwapUsdcLollyEvent.fromDecoded(val["_0"])])
  }
  if ("ProcessWinningNumbers" in obj) {
    const val = obj["ProcessWinningNumbers"]
    return new ProcessWinningNumbers([
      types.ProcessWinningNumbersEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("RequestWinningNumbers" in obj) {
    const val = obj["RequestWinningNumbers"]
    return new RequestWinningNumbers([
      types.RequestWinningNumbersEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("TestEmitWinningNumbers" in obj) {
    const val = obj["TestEmitWinningNumbers"]
    return new TestEmitWinningNumbers([
      types.TestEmitWinningNumbersEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("BuyLottoTicket" in obj) {
    const val = obj["BuyLottoTicket"]
    return new BuyLottoTicket([
      types.BuyLottoTicketEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("ClaimUserRewards" in obj) {
    const val = obj["ClaimUserRewards"]
    return new ClaimUserRewards([
      types.ClaimUserRewardsEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CloseLottoTicket" in obj) {
    const val = obj["CloseLottoTicket"]
    return new CloseLottoTicket([
      types.CloseLottoTicketEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CloseUserMetadata" in obj) {
    const val = obj["CloseUserMetadata"]
    return new CloseUserMetadata([
      types.CloseUserMetadataEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CreateUserMetadata" in obj) {
    const val = obj["CreateUserMetadata"]
    return new CreateUserMetadata([
      types.CreateUserMetadataEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CrankLottoGameClosed" in obj) {
    const val = obj["CrankLottoGameClosed"]
    return new CrankLottoGameClosed([
      types.CrankLottoGameClosedEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CrankLottoGameWinners" in obj) {
    const val = obj["CrankLottoGameWinners"]
    return new CrankLottoGameWinners([
      types.CrankLottoGameWinnersEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CrankTransferWinningAmountToUserRewardsVault" in obj) {
    const val = obj["CrankTransferWinningAmountToUserRewardsVault"]
    return new CrankTransferWinningAmountToUserRewardsVault([
      types.CrankTransferWinningAmountToUserRewardsVaultEvent.fromDecoded(
        val["_0"]
      ),
    ])
  }
  if ("CrankTransferToBuyAndBurnVault" in obj) {
    const val = obj["CrankTransferToBuyAndBurnVault"]
    return new CrankTransferToBuyAndBurnVault([
      types.CrankTransferToBuyAndBurnVaultEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("DuplicateWinningNumbers" in obj) {
    const val = obj["DuplicateWinningNumbers"]
    return new DuplicateWinningNumbers([
      types.DuplicateWinningNumbersEvent.fromDecoded(val["_0"]),
    ])
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(
  obj: types.LollysLottoProgramEventDataJSON
): types.LollysLottoProgramEventDataKind {
  switch (obj.kind) {
    case "BurnLolly": {
      return new BurnLolly([types.BurnLollyEvent.fromJSON(obj.value[0])])
    }
    case "CloseEventEmitter": {
      return new CloseEventEmitter([
        types.CloseEventEmitterEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CloseLollyBurnState": {
      return new CloseLollyBurnState([
        types.CloseLollyBurnStateEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CloseLollysLotto": {
      return new CloseLollysLotto([
        types.CloseLollysLottoEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CloseLottoGame": {
      return new CloseLottoGame([
        types.CloseLottoGameEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CreateLollyBurnState": {
      return new CreateLollyBurnState([
        types.CreateLollyBurnStateEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CreateLollysLotto": {
      return new CreateLollysLotto([
        types.CreateLollysLottoEvent.fromJSON(obj.value[0]),
      ])
    }
    case "StartLottoGame": {
      return new StartLottoGame([
        types.StartLottoGameEvent.fromJSON(obj.value[0]),
      ])
    }
    case "SwapUsdcLolly": {
      return new SwapUsdcLolly([
        types.SwapUsdcLollyEvent.fromJSON(obj.value[0]),
      ])
    }
    case "ProcessWinningNumbers": {
      return new ProcessWinningNumbers([
        types.ProcessWinningNumbersEvent.fromJSON(obj.value[0]),
      ])
    }
    case "RequestWinningNumbers": {
      return new RequestWinningNumbers([
        types.RequestWinningNumbersEvent.fromJSON(obj.value[0]),
      ])
    }
    case "TestEmitWinningNumbers": {
      return new TestEmitWinningNumbers([
        types.TestEmitWinningNumbersEvent.fromJSON(obj.value[0]),
      ])
    }
    case "BuyLottoTicket": {
      return new BuyLottoTicket([
        types.BuyLottoTicketEvent.fromJSON(obj.value[0]),
      ])
    }
    case "ClaimUserRewards": {
      return new ClaimUserRewards([
        types.ClaimUserRewardsEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CloseLottoTicket": {
      return new CloseLottoTicket([
        types.CloseLottoTicketEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CloseUserMetadata": {
      return new CloseUserMetadata([
        types.CloseUserMetadataEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CreateUserMetadata": {
      return new CreateUserMetadata([
        types.CreateUserMetadataEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CrankLottoGameClosed": {
      return new CrankLottoGameClosed([
        types.CrankLottoGameClosedEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CrankLottoGameWinners": {
      return new CrankLottoGameWinners([
        types.CrankLottoGameWinnersEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CrankTransferWinningAmountToUserRewardsVault": {
      return new CrankTransferWinningAmountToUserRewardsVault([
        types.CrankTransferWinningAmountToUserRewardsVaultEvent.fromJSON(
          obj.value[0]
        ),
      ])
    }
    case "CrankTransferToBuyAndBurnVault": {
      return new CrankTransferToBuyAndBurnVault([
        types.CrankTransferToBuyAndBurnVaultEvent.fromJSON(obj.value[0]),
      ])
    }
    case "DuplicateWinningNumbers": {
      return new DuplicateWinningNumbers([
        types.DuplicateWinningNumbersEvent.fromJSON(obj.value[0]),
      ])
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([types.BurnLollyEvent.layout("_0")], "BurnLolly"),
    borsh.struct(
      [types.CloseEventEmitterEvent.layout("_0")],
      "CloseEventEmitter"
    ),
    borsh.struct(
      [types.CloseLollyBurnStateEvent.layout("_0")],
      "CloseLollyBurnState"
    ),
    borsh.struct(
      [types.CloseLollysLottoEvent.layout("_0")],
      "CloseLollysLotto"
    ),
    borsh.struct([types.CloseLottoGameEvent.layout("_0")], "CloseLottoGame"),
    borsh.struct(
      [types.CreateLollyBurnStateEvent.layout("_0")],
      "CreateLollyBurnState"
    ),
    borsh.struct(
      [types.CreateLollysLottoEvent.layout("_0")],
      "CreateLollysLotto"
    ),
    borsh.struct([types.StartLottoGameEvent.layout("_0")], "StartLottoGame"),
    borsh.struct([types.SwapUsdcLollyEvent.layout("_0")], "SwapUsdcLolly"),
    borsh.struct(
      [types.ProcessWinningNumbersEvent.layout("_0")],
      "ProcessWinningNumbers"
    ),
    borsh.struct(
      [types.RequestWinningNumbersEvent.layout("_0")],
      "RequestWinningNumbers"
    ),
    borsh.struct(
      [types.TestEmitWinningNumbersEvent.layout("_0")],
      "TestEmitWinningNumbers"
    ),
    borsh.struct([types.BuyLottoTicketEvent.layout("_0")], "BuyLottoTicket"),
    borsh.struct(
      [types.ClaimUserRewardsEvent.layout("_0")],
      "ClaimUserRewards"
    ),
    borsh.struct(
      [types.CloseLottoTicketEvent.layout("_0")],
      "CloseLottoTicket"
    ),
    borsh.struct(
      [types.CloseUserMetadataEvent.layout("_0")],
      "CloseUserMetadata"
    ),
    borsh.struct(
      [types.CreateUserMetadataEvent.layout("_0")],
      "CreateUserMetadata"
    ),
    borsh.struct(
      [types.CrankLottoGameClosedEvent.layout("_0")],
      "CrankLottoGameClosed"
    ),
    borsh.struct(
      [types.CrankLottoGameWinnersEvent.layout("_0")],
      "CrankLottoGameWinners"
    ),
    borsh.struct(
      [types.CrankTransferWinningAmountToUserRewardsVaultEvent.layout("_0")],
      "CrankTransferWinningAmountToUserRewardsVault"
    ),
    borsh.struct(
      [types.CrankTransferToBuyAndBurnVaultEvent.layout("_0")],
      "CrankTransferToBuyAndBurnVault"
    ),
    borsh.struct(
      [types.DuplicateWinningNumbersEvent.layout("_0")],
      "DuplicateWinningNumbers"
    ),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
