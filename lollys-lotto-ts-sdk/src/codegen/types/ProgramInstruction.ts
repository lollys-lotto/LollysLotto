import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface ProcessWinningNumbersJSON {
  kind: "ProcessWinningNumbers"
}

export class ProcessWinningNumbers {
  static readonly discriminator = 0
  static readonly kind = "ProcessWinningNumbers"
  readonly discriminator = 0
  readonly kind = "ProcessWinningNumbers"

  toJSON(): ProcessWinningNumbersJSON {
    return {
      kind: "ProcessWinningNumbers",
    }
  }

  toEncodable() {
    return {
      ProcessWinningNumbers: {},
    }
  }
}

export interface RequestWinningNumbersJSON {
  kind: "RequestWinningNumbers"
}

export class RequestWinningNumbers {
  static readonly discriminator = 1
  static readonly kind = "RequestWinningNumbers"
  readonly discriminator = 1
  readonly kind = "RequestWinningNumbers"

  toJSON(): RequestWinningNumbersJSON {
    return {
      kind: "RequestWinningNumbers",
    }
  }

  toEncodable() {
    return {
      RequestWinningNumbers: {},
    }
  }
}

export interface BurnLollyJSON {
  kind: "BurnLolly"
}

export class BurnLolly {
  static readonly discriminator = 2
  static readonly kind = "BurnLolly"
  readonly discriminator = 2
  readonly kind = "BurnLolly"

  toJSON(): BurnLollyJSON {
    return {
      kind: "BurnLolly",
    }
  }

  toEncodable() {
    return {
      BurnLolly: {},
    }
  }
}

export interface BuyLottoTicketJSON {
  kind: "BuyLottoTicket"
}

export class BuyLottoTicket {
  static readonly discriminator = 3
  static readonly kind = "BuyLottoTicket"
  readonly discriminator = 3
  readonly kind = "BuyLottoTicket"

  toJSON(): BuyLottoTicketJSON {
    return {
      kind: "BuyLottoTicket",
    }
  }

  toEncodable() {
    return {
      BuyLottoTicket: {},
    }
  }
}

export interface CrankLottoGameWinnerJSON {
  kind: "CrankLottoGameWinner"
}

export class CrankLottoGameWinner {
  static readonly discriminator = 4
  static readonly kind = "CrankLottoGameWinner"
  readonly discriminator = 4
  readonly kind = "CrankLottoGameWinner"

  toJSON(): CrankLottoGameWinnerJSON {
    return {
      kind: "CrankLottoGameWinner",
    }
  }

  toEncodable() {
    return {
      CrankLottoGameWinner: {},
    }
  }
}

export interface CreateLollyBurnStateJSON {
  kind: "CreateLollyBurnState"
}

export class CreateLollyBurnState {
  static readonly discriminator = 5
  static readonly kind = "CreateLollyBurnState"
  readonly discriminator = 5
  readonly kind = "CreateLollyBurnState"

  toJSON(): CreateLollyBurnStateJSON {
    return {
      kind: "CreateLollyBurnState",
    }
  }

  toEncodable() {
    return {
      CreateLollyBurnState: {},
    }
  }
}

export interface CreateLollysLottoJSON {
  kind: "CreateLollysLotto"
}

export class CreateLollysLotto {
  static readonly discriminator = 6
  static readonly kind = "CreateLollysLotto"
  readonly discriminator = 6
  readonly kind = "CreateLollysLotto"

  toJSON(): CreateLollysLottoJSON {
    return {
      kind: "CreateLollysLotto",
    }
  }

  toEncodable() {
    return {
      CreateLollysLotto: {},
    }
  }
}

export interface CreateUserMetadataJSON {
  kind: "CreateUserMetadata"
}

export class CreateUserMetadata {
  static readonly discriminator = 7
  static readonly kind = "CreateUserMetadata"
  readonly discriminator = 7
  readonly kind = "CreateUserMetadata"

  toJSON(): CreateUserMetadataJSON {
    return {
      kind: "CreateUserMetadata",
    }
  }

  toEncodable() {
    return {
      CreateUserMetadata: {},
    }
  }
}

export interface StartLottoGameJSON {
  kind: "StartLottoGame"
}

export class StartLottoGame {
  static readonly discriminator = 8
  static readonly kind = "StartLottoGame"
  readonly discriminator = 8
  readonly kind = "StartLottoGame"

  toJSON(): StartLottoGameJSON {
    return {
      kind: "StartLottoGame",
    }
  }

  toEncodable() {
    return {
      StartLottoGame: {},
    }
  }
}

export interface SwapUsdcLollyJSON {
  kind: "SwapUsdcLolly"
}

export class SwapUsdcLolly {
  static readonly discriminator = 9
  static readonly kind = "SwapUsdcLolly"
  readonly discriminator = 9
  readonly kind = "SwapUsdcLolly"

  toJSON(): SwapUsdcLollyJSON {
    return {
      kind: "SwapUsdcLolly",
    }
  }

  toEncodable() {
    return {
      SwapUsdcLolly: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.ProgramInstructionKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("ProcessWinningNumbers" in obj) {
    return new ProcessWinningNumbers()
  }
  if ("RequestWinningNumbers" in obj) {
    return new RequestWinningNumbers()
  }
  if ("BurnLolly" in obj) {
    return new BurnLolly()
  }
  if ("BuyLottoTicket" in obj) {
    return new BuyLottoTicket()
  }
  if ("CrankLottoGameWinner" in obj) {
    return new CrankLottoGameWinner()
  }
  if ("CreateLollyBurnState" in obj) {
    return new CreateLollyBurnState()
  }
  if ("CreateLollysLotto" in obj) {
    return new CreateLollysLotto()
  }
  if ("CreateUserMetadata" in obj) {
    return new CreateUserMetadata()
  }
  if ("StartLottoGame" in obj) {
    return new StartLottoGame()
  }
  if ("SwapUsdcLolly" in obj) {
    return new SwapUsdcLolly()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(
  obj: types.ProgramInstructionJSON
): types.ProgramInstructionKind {
  switch (obj.kind) {
    case "ProcessWinningNumbers": {
      return new ProcessWinningNumbers()
    }
    case "RequestWinningNumbers": {
      return new RequestWinningNumbers()
    }
    case "BurnLolly": {
      return new BurnLolly()
    }
    case "BuyLottoTicket": {
      return new BuyLottoTicket()
    }
    case "CrankLottoGameWinner": {
      return new CrankLottoGameWinner()
    }
    case "CreateLollyBurnState": {
      return new CreateLollyBurnState()
    }
    case "CreateLollysLotto": {
      return new CreateLollysLotto()
    }
    case "CreateUserMetadata": {
      return new CreateUserMetadata()
    }
    case "StartLottoGame": {
      return new StartLottoGame()
    }
    case "SwapUsdcLolly": {
      return new SwapUsdcLolly()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "ProcessWinningNumbers"),
    borsh.struct([], "RequestWinningNumbers"),
    borsh.struct([], "BurnLolly"),
    borsh.struct([], "BuyLottoTicket"),
    borsh.struct([], "CrankLottoGameWinner"),
    borsh.struct([], "CreateLollyBurnState"),
    borsh.struct([], "CreateLollysLotto"),
    borsh.struct([], "CreateUserMetadata"),
    borsh.struct([], "StartLottoGame"),
    borsh.struct([], "SwapUsdcLolly"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
