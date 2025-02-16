import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface RequestWinningNumbersEventFields {
  lottoGame: PublicKey
  round: BN
}

export interface RequestWinningNumbersEventJSON {
  lottoGame: string
  round: string
}

/** Event emitted when a user requests randomness. */
export class RequestWinningNumbersEvent {
  readonly lottoGame: PublicKey
  readonly round: BN

  constructor(fields: RequestWinningNumbersEventFields) {
    this.lottoGame = fields.lottoGame
    this.round = fields.round
  }

  static layout(property?: string) {
    return borsh.struct(
      [borsh.publicKey("lottoGame"), borsh.u64("round")],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new RequestWinningNumbersEvent({
      lottoGame: obj.lottoGame,
      round: obj.round,
    })
  }

  static toEncodable(fields: RequestWinningNumbersEventFields) {
    return {
      lottoGame: fields.lottoGame,
      round: fields.round,
    }
  }

  toJSON(): RequestWinningNumbersEventJSON {
    return {
      lottoGame: this.lottoGame.toString(),
      round: this.round.toString(),
    }
  }

  static fromJSON(
    obj: RequestWinningNumbersEventJSON
  ): RequestWinningNumbersEvent {
    return new RequestWinningNumbersEvent({
      lottoGame: new PublicKey(obj.lottoGame),
      round: new BN(obj.round),
    })
  }

  toEncodable() {
    return RequestWinningNumbersEvent.toEncodable(this)
  }
}
