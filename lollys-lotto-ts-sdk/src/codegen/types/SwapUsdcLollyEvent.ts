import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface SwapUsdcLollyEventFields {
  user: PublicKey
  usdc: PublicKey
  lolly: PublicKey
}

export interface SwapUsdcLollyEventJSON {
  user: string
  usdc: string
  lolly: string
}

/** Event emitted when a user swaps USDC for lolly. */
export class SwapUsdcLollyEvent {
  readonly user: PublicKey
  readonly usdc: PublicKey
  readonly lolly: PublicKey

  constructor(fields: SwapUsdcLollyEventFields) {
    this.user = fields.user
    this.usdc = fields.usdc
    this.lolly = fields.lolly
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("user"),
        borsh.publicKey("usdc"),
        borsh.publicKey("lolly"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new SwapUsdcLollyEvent({
      user: obj.user,
      usdc: obj.usdc,
      lolly: obj.lolly,
    })
  }

  static toEncodable(fields: SwapUsdcLollyEventFields) {
    return {
      user: fields.user,
      usdc: fields.usdc,
      lolly: fields.lolly,
    }
  }

  toJSON(): SwapUsdcLollyEventJSON {
    return {
      user: this.user.toString(),
      usdc: this.usdc.toString(),
      lolly: this.lolly.toString(),
    }
  }

  static fromJSON(obj: SwapUsdcLollyEventJSON): SwapUsdcLollyEvent {
    return new SwapUsdcLollyEvent({
      user: new PublicKey(obj.user),
      usdc: new PublicKey(obj.usdc),
      lolly: new PublicKey(obj.lolly),
    })
  }

  toEncodable() {
    return SwapUsdcLollyEvent.toEncodable(this)
  }
}
