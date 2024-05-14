import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CloseLollyBurnStateEventFields {
  lollyBurnState: PublicKey
}

export interface CloseLollyBurnStateEventJSON {
  lollyBurnState: string
}

/** Event emitted when a user closes a LollyBurnState. */
export class CloseLollyBurnStateEvent {
  readonly lollyBurnState: PublicKey

  constructor(fields: CloseLollyBurnStateEventFields) {
    this.lollyBurnState = fields.lollyBurnState
  }

  static layout(property?: string) {
    return borsh.struct([borsh.publicKey("lollyBurnState")], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CloseLollyBurnStateEvent({
      lollyBurnState: obj.lollyBurnState,
    })
  }

  static toEncodable(fields: CloseLollyBurnStateEventFields) {
    return {
      lollyBurnState: fields.lollyBurnState,
    }
  }

  toJSON(): CloseLollyBurnStateEventJSON {
    return {
      lollyBurnState: this.lollyBurnState.toString(),
    }
  }

  static fromJSON(obj: CloseLollyBurnStateEventJSON): CloseLollyBurnStateEvent {
    return new CloseLollyBurnStateEvent({
      lollyBurnState: new PublicKey(obj.lollyBurnState),
    })
  }

  toEncodable() {
    return CloseLollyBurnStateEvent.toEncodable(this)
  }
}
