import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface NotUpdatedJSON {
  kind: "NotUpdated"
}

export class NotUpdated {
  static readonly discriminator = 0
  static readonly kind = "NotUpdated"
  readonly discriminator = 0
  readonly kind = "NotUpdated"

  toJSON(): NotUpdatedJSON {
    return {
      kind: "NotUpdated",
    }
  }

  toEncodable() {
    return {
      NotUpdated: {},
    }
  }
}

export interface UpdatedJSON {
  kind: "Updated"
}

export class Updated {
  static readonly discriminator = 1
  static readonly kind = "Updated"
  readonly discriminator = 1
  readonly kind = "Updated"

  toJSON(): UpdatedJSON {
    return {
      kind: "Updated",
    }
  }

  toEncodable() {
    return {
      Updated: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.WinningNumberUpdateStateKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("NotUpdated" in obj) {
    return new NotUpdated()
  }
  if ("Updated" in obj) {
    return new Updated()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(
  obj: types.WinningNumberUpdateStateJSON
): types.WinningNumberUpdateStateKind {
  switch (obj.kind) {
    case "NotUpdated": {
      return new NotUpdated()
    }
    case "Updated": {
      return new Updated()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "NotUpdated"),
    borsh.struct([], "Updated"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
