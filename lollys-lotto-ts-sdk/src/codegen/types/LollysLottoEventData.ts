import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export type ProcessWinningNumbersFields = [
  types.ProcessWinningNumbersEventFields
]
export type ProcessWinningNumbersValue = [types.ProcessWinningNumbersEvent]

export interface ProcessWinningNumbersJSON {
  kind: "ProcessWinningNumbers"
  value: [types.ProcessWinningNumbersEventJSON]
}

export class ProcessWinningNumbers {
  static readonly discriminator = 0
  static readonly kind = "ProcessWinningNumbers"
  readonly discriminator = 0
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
  static readonly discriminator = 1
  static readonly kind = "RequestWinningNumbers"
  readonly discriminator = 1
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

export type BurnLollyFields = [types.BurnLollyEventFields]
export type BurnLollyValue = [types.BurnLollyEvent]

export interface BurnLollyJSON {
  kind: "BurnLolly"
  value: [types.BurnLollyEventJSON]
}

export class BurnLolly {
  static readonly discriminator = 2
  static readonly kind = "BurnLolly"
  readonly discriminator = 2
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

export type BuyLottoTicketFields = [types.BuyLottoTicketEventFields]
export type BuyLottoTicketValue = [types.BuyLottoTicketEvent]

export interface BuyLottoTicketJSON {
  kind: "BuyLottoTicket"
  value: [types.BuyLottoTicketEventJSON]
}

export class BuyLottoTicket {
  static readonly discriminator = 3
  static readonly kind = "BuyLottoTicket"
  readonly discriminator = 3
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

export type CrankLottoGameWinnerFields = [types.CrankLottoGameWinnerEventFields]
export type CrankLottoGameWinnerValue = [types.CrankLottoGameWinnerEvent]

export interface CrankLottoGameWinnerJSON {
  kind: "CrankLottoGameWinner"
  value: [types.CrankLottoGameWinnerEventJSON]
}

export class CrankLottoGameWinner {
  static readonly discriminator = 4
  static readonly kind = "CrankLottoGameWinner"
  readonly discriminator = 4
  readonly kind = "CrankLottoGameWinner"
  readonly value: CrankLottoGameWinnerValue

  constructor(value: CrankLottoGameWinnerFields) {
    this.value = [new types.CrankLottoGameWinnerEvent({ ...value[0] })]
  }

  toJSON(): CrankLottoGameWinnerJSON {
    return {
      kind: "CrankLottoGameWinner",
      value: [this.value[0].toJSON()],
    }
  }

  toEncodable() {
    return {
      CrankLottoGameWinner: {
        _0: types.CrankLottoGameWinnerEvent.toEncodable(this.value[0]),
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

export type CreateUserMetadataFields = [types.CreateUserMetadataEventFields]
export type CreateUserMetadataValue = [types.CreateUserMetadataEvent]

export interface CreateUserMetadataJSON {
  kind: "CreateUserMetadata"
  value: [types.CreateUserMetadataEventJSON]
}

export class CreateUserMetadata {
  static readonly discriminator = 7
  static readonly kind = "CreateUserMetadata"
  readonly discriminator = 7
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

export type StartLottoGameFields = [types.StartLottoGameEventFields]
export type StartLottoGameValue = [types.StartLottoGameEvent]

export interface StartLottoGameJSON {
  kind: "StartLottoGame"
  value: [types.StartLottoGameEventJSON]
}

export class StartLottoGame {
  static readonly discriminator = 8
  static readonly kind = "StartLottoGame"
  readonly discriminator = 8
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
  static readonly discriminator = 9
  static readonly kind = "SwapUsdcLolly"
  readonly discriminator = 9
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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.LollysLottoEventDataKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
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
  if ("BurnLolly" in obj) {
    const val = obj["BurnLolly"]
    return new BurnLolly([types.BurnLollyEvent.fromDecoded(val["_0"])])
  }
  if ("BuyLottoTicket" in obj) {
    const val = obj["BuyLottoTicket"]
    return new BuyLottoTicket([
      types.BuyLottoTicketEvent.fromDecoded(val["_0"]),
    ])
  }
  if ("CrankLottoGameWinner" in obj) {
    const val = obj["CrankLottoGameWinner"]
    return new CrankLottoGameWinner([
      types.CrankLottoGameWinnerEvent.fromDecoded(val["_0"]),
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
  if ("CreateUserMetadata" in obj) {
    const val = obj["CreateUserMetadata"]
    return new CreateUserMetadata([
      types.CreateUserMetadataEvent.fromDecoded(val["_0"]),
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

  throw new Error("Invalid enum object")
}

export function fromJSON(
  obj: types.LollysLottoEventDataJSON
): types.LollysLottoEventDataKind {
  switch (obj.kind) {
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
    case "BurnLolly": {
      return new BurnLolly([types.BurnLollyEvent.fromJSON(obj.value[0])])
    }
    case "BuyLottoTicket": {
      return new BuyLottoTicket([
        types.BuyLottoTicketEvent.fromJSON(obj.value[0]),
      ])
    }
    case "CrankLottoGameWinner": {
      return new CrankLottoGameWinner([
        types.CrankLottoGameWinnerEvent.fromJSON(obj.value[0]),
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
    case "CreateUserMetadata": {
      return new CreateUserMetadata([
        types.CreateUserMetadataEvent.fromJSON(obj.value[0]),
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
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct(
      [types.ProcessWinningNumbersEvent.layout("_0")],
      "ProcessWinningNumbers"
    ),
    borsh.struct(
      [types.RequestWinningNumbersEvent.layout("_0")],
      "RequestWinningNumbers"
    ),
    borsh.struct([types.BurnLollyEvent.layout("_0")], "BurnLolly"),
    borsh.struct([types.BuyLottoTicketEvent.layout("_0")], "BuyLottoTicket"),
    borsh.struct(
      [types.CrankLottoGameWinnerEvent.layout("_0")],
      "CrankLottoGameWinner"
    ),
    borsh.struct(
      [types.CreateLollyBurnStateEvent.layout("_0")],
      "CreateLollyBurnState"
    ),
    borsh.struct(
      [types.CreateLollysLottoEvent.layout("_0")],
      "CreateLollysLotto"
    ),
    borsh.struct(
      [types.CreateUserMetadataEvent.layout("_0")],
      "CreateUserMetadata"
    ),
    borsh.struct([types.StartLottoGameEvent.layout("_0")], "StartLottoGame"),
    borsh.struct([types.SwapUsdcLollyEvent.layout("_0")], "SwapUsdcLolly"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
