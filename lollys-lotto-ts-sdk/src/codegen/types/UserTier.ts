import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface BronzeJSON {
  kind: "Bronze"
}

export class Bronze {
  static readonly discriminator = 0
  static readonly kind = "Bronze"
  readonly discriminator = 0
  readonly kind = "Bronze"

  toJSON(): BronzeJSON {
    return {
      kind: "Bronze",
    }
  }

  toEncodable() {
    return {
      Bronze: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.UserTierKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("Bronze" in obj) {
    return new Bronze()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(obj: types.UserTierJSON): types.UserTierKind {
  switch (obj.kind) {
    case "Bronze": {
      return new Bronze()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([borsh.struct([], "Bronze")])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
