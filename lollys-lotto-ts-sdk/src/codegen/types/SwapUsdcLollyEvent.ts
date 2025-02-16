import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface SwapUsdcLollyEventFields {
  authority: PublicKey
  lollyBurnState: PublicKey
}

export interface SwapUsdcLollyEventJSON {
  authority: string
  lollyBurnState: string
}

/** Event emitted when a user swaps USDC for lolly. */
export class SwapUsdcLollyEvent {
  readonly authority: PublicKey
  readonly lollyBurnState: PublicKey

  constructor(fields: SwapUsdcLollyEventFields) {
    this.authority = fields.authority
    this.lollyBurnState = fields.lollyBurnState
  }

  static layout(property?: string) {
    return borsh.struct(
      [borsh.publicKey("authority"), borsh.publicKey("lollyBurnState")],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new SwapUsdcLollyEvent({
      authority: obj.authority,
      lollyBurnState: obj.lollyBurnState,
    })
  }

  static toEncodable(fields: SwapUsdcLollyEventFields) {
    return {
      authority: fields.authority,
      lollyBurnState: fields.lollyBurnState,
    }
  }

  toJSON(): SwapUsdcLollyEventJSON {
    return {
      authority: this.authority.toString(),
      lollyBurnState: this.lollyBurnState.toString(),
    }
  }

  static fromJSON(obj: SwapUsdcLollyEventJSON): SwapUsdcLollyEvent {
    return new SwapUsdcLollyEvent({
      authority: new PublicKey(obj.authority),
      lollyBurnState: new PublicKey(obj.lollyBurnState),
    })
  }

  toEncodable() {
    return SwapUsdcLollyEvent.toEncodable(this)
  }
}
