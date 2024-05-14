import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface NotDisbursedJSON {
  kind: "NotDisbursed"
}

export class NotDisbursed {
  static readonly discriminator = 0
  static readonly kind = "NotDisbursed"
  readonly discriminator = 0
  readonly kind = "NotDisbursed"

  toJSON(): NotDisbursedJSON {
    return {
      kind: "NotDisbursed",
    }
  }

  toEncodable() {
    return {
      NotDisbursed: {},
    }
  }
}

export interface DisbursedJSON {
  kind: "Disbursed"
}

export class Disbursed {
  static readonly discriminator = 1
  static readonly kind = "Disbursed"
  readonly discriminator = 1
  readonly kind = "Disbursed"

  toJSON(): DisbursedJSON {
    return {
      kind: "Disbursed",
    }
  }

  toEncodable() {
    return {
      Disbursed: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.WinningAmountDisbursedStateKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("NotDisbursed" in obj) {
    return new NotDisbursed()
  }
  if ("Disbursed" in obj) {
    return new Disbursed()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(
  obj: types.WinningAmountDisbursedStateJSON
): types.WinningAmountDisbursedStateKind {
  switch (obj.kind) {
    case "NotDisbursed": {
      return new NotDisbursed()
    }
    case "Disbursed": {
      return new Disbursed()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "NotDisbursed"),
    borsh.struct([], "Disbursed"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
