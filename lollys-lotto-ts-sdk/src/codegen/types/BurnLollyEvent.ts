import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface BurnLollyEventFields {
  user: PublicKey
  lolly: PublicKey
}

export interface BurnLollyEventJSON {
  user: string
  lolly: string
}

/** Event emitted when a user burns a lolly. */
export class BurnLollyEvent {
  readonly user: PublicKey
  readonly lolly: PublicKey

  constructor(fields: BurnLollyEventFields) {
    this.user = fields.user
    this.lolly = fields.lolly
  }

  static layout(property?: string) {
    return borsh.struct(
      [borsh.publicKey("user"), borsh.publicKey("lolly")],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new BurnLollyEvent({
      user: obj.user,
      lolly: obj.lolly,
    })
  }

  static toEncodable(fields: BurnLollyEventFields) {
    return {
      user: fields.user,
      lolly: fields.lolly,
    }
  }

  toJSON(): BurnLollyEventJSON {
    return {
      user: this.user.toString(),
      lolly: this.lolly.toString(),
    }
  }

  static fromJSON(obj: BurnLollyEventJSON): BurnLollyEvent {
    return new BurnLollyEvent({
      user: new PublicKey(obj.user),
      lolly: new PublicKey(obj.lolly),
    })
  }

  toEncodable() {
    return BurnLollyEvent.toEncodable(this)
  }
}
