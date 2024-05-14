import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CloseLottoGameEventFields {
  lottoGame: PublicKey
  round: BN
}

export interface CloseLottoGameEventJSON {
  lottoGame: string
  round: string
}

export class CloseLottoGameEvent {
  readonly lottoGame: PublicKey
  readonly round: BN

  constructor(fields: CloseLottoGameEventFields) {
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
    return new CloseLottoGameEvent({
      lottoGame: obj.lottoGame,
      round: obj.round,
    })
  }

  static toEncodable(fields: CloseLottoGameEventFields) {
    return {
      lottoGame: fields.lottoGame,
      round: fields.round,
    }
  }

  toJSON(): CloseLottoGameEventJSON {
    return {
      lottoGame: this.lottoGame.toString(),
      round: this.round.toString(),
    }
  }

  static fromJSON(obj: CloseLottoGameEventJSON): CloseLottoGameEvent {
    return new CloseLottoGameEvent({
      lottoGame: new PublicKey(obj.lottoGame),
      round: new BN(obj.round),
    })
  }

  toEncodable() {
    return CloseLottoGameEvent.toEncodable(this)
  }
}
