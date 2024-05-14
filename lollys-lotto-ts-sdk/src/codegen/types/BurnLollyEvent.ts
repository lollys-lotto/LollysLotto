import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface BurnLollyEventFields {
  authority: PublicKey
  lollyBurnState: PublicKey
  lollyBurntAmountNow: BN
  totalLollyBurnt: BN
}

export interface BurnLollyEventJSON {
  authority: string
  lollyBurnState: string
  lollyBurntAmountNow: string
  totalLollyBurnt: string
}

/** Event emitted when a user burns $LOLLY tokens. */
export class BurnLollyEvent {
  readonly authority: PublicKey
  readonly lollyBurnState: PublicKey
  readonly lollyBurntAmountNow: BN
  readonly totalLollyBurnt: BN

  constructor(fields: BurnLollyEventFields) {
    this.authority = fields.authority
    this.lollyBurnState = fields.lollyBurnState
    this.lollyBurntAmountNow = fields.lollyBurntAmountNow
    this.totalLollyBurnt = fields.totalLollyBurnt
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("authority"),
        borsh.publicKey("lollyBurnState"),
        borsh.u64("lollyBurntAmountNow"),
        borsh.u64("totalLollyBurnt"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new BurnLollyEvent({
      authority: obj.authority,
      lollyBurnState: obj.lollyBurnState,
      lollyBurntAmountNow: obj.lollyBurntAmountNow,
      totalLollyBurnt: obj.totalLollyBurnt,
    })
  }

  static toEncodable(fields: BurnLollyEventFields) {
    return {
      authority: fields.authority,
      lollyBurnState: fields.lollyBurnState,
      lollyBurntAmountNow: fields.lollyBurntAmountNow,
      totalLollyBurnt: fields.totalLollyBurnt,
    }
  }

  toJSON(): BurnLollyEventJSON {
    return {
      authority: this.authority.toString(),
      lollyBurnState: this.lollyBurnState.toString(),
      lollyBurntAmountNow: this.lollyBurntAmountNow.toString(),
      totalLollyBurnt: this.totalLollyBurnt.toString(),
    }
  }

  static fromJSON(obj: BurnLollyEventJSON): BurnLollyEvent {
    return new BurnLollyEvent({
      authority: new PublicKey(obj.authority),
      lollyBurnState: new PublicKey(obj.lollyBurnState),
      lollyBurntAmountNow: new BN(obj.lollyBurntAmountNow),
      totalLollyBurnt: new BN(obj.totalLollyBurnt),
    })
  }

  toEncodable() {
    return BurnLollyEvent.toEncodable(this)
  }
}
