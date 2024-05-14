import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface NotStartedJSON {
  kind: "NotStarted"
}

export class NotStarted {
  static readonly discriminator = 0
  static readonly kind = "NotStarted"
  readonly discriminator = 0
  readonly kind = "NotStarted"

  toJSON(): NotStartedJSON {
    return {
      kind: "NotStarted",
    }
  }

  toEncodable() {
    return {
      NotStarted: {},
    }
  }
}

export interface OpenJSON {
  kind: "Open"
}

export class Open {
  static readonly discriminator = 1
  static readonly kind = "Open"
  readonly discriminator = 1
  readonly kind = "Open"

  toJSON(): OpenJSON {
    return {
      kind: "Open",
    }
  }

  toEncodable() {
    return {
      Open: {},
    }
  }
}

export interface ClosedJSON {
  kind: "Closed"
}

export class Closed {
  static readonly discriminator = 2
  static readonly kind = "Closed"
  readonly discriminator = 2
  readonly kind = "Closed"

  toJSON(): ClosedJSON {
    return {
      kind: "Closed",
    }
  }

  toEncodable() {
    return {
      Closed: {},
    }
  }
}

export interface FinishedJSON {
  kind: "Finished"
}

export class Finished {
  static readonly discriminator = 3
  static readonly kind = "Finished"
  readonly discriminator = 3
  readonly kind = "Finished"

  toJSON(): FinishedJSON {
    return {
      kind: "Finished",
    }
  }

  toEncodable() {
    return {
      Finished: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.LottoGameStateKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("NotStarted" in obj) {
    return new NotStarted()
  }
  if ("Open" in obj) {
    return new Open()
  }
  if ("Closed" in obj) {
    return new Closed()
  }
  if ("Finished" in obj) {
    return new Finished()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(
  obj: types.LottoGameStateJSON
): types.LottoGameStateKind {
  switch (obj.kind) {
    case "NotStarted": {
      return new NotStarted()
    }
    case "Open": {
      return new Open()
    }
    case "Closed": {
      return new Closed()
    }
    case "Finished": {
      return new Finished()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "NotStarted"),
    borsh.struct([], "Open"),
    borsh.struct([], "Closed"),
    borsh.struct([], "Finished"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
