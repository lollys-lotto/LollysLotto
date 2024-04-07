import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface RequestWinningNumbersEventFields {
  round: BN
}

export interface RequestWinningNumbersEventJSON {
  round: string
}

/** Event emitted when a user requests randomness. */
export class RequestWinningNumbersEvent {
  readonly round: BN

  constructor(fields: RequestWinningNumbersEventFields) {
    this.round = fields.round
  }

  static layout(property?: string) {
    return borsh.struct([borsh.u64("round")], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new RequestWinningNumbersEvent({
      round: obj.round,
    })
  }

  static toEncodable(fields: RequestWinningNumbersEventFields) {
    return {
      round: fields.round,
    }
  }

  toJSON(): RequestWinningNumbersEventJSON {
    return {
      round: this.round.toString(),
    }
  }

  static fromJSON(
    obj: RequestWinningNumbersEventJSON
  ): RequestWinningNumbersEvent {
    return new RequestWinningNumbersEvent({
      round: new BN(obj.round),
    })
  }

  toEncodable() {
    return RequestWinningNumbersEvent.toEncodable(this)
  }
}
