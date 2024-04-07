import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CrankLottoGameWinnerEventFields {
  user: PublicKey
  game: PublicKey
  winner: PublicKey
}

export interface CrankLottoGameWinnerEventJSON {
  user: string
  game: string
  winner: string
}

/** Event emitted when a user cranks a lotto game winner. */
export class CrankLottoGameWinnerEvent {
  readonly user: PublicKey
  readonly game: PublicKey
  readonly winner: PublicKey

  constructor(fields: CrankLottoGameWinnerEventFields) {
    this.user = fields.user
    this.game = fields.game
    this.winner = fields.winner
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("user"),
        borsh.publicKey("game"),
        borsh.publicKey("winner"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CrankLottoGameWinnerEvent({
      user: obj.user,
      game: obj.game,
      winner: obj.winner,
    })
  }

  static toEncodable(fields: CrankLottoGameWinnerEventFields) {
    return {
      user: fields.user,
      game: fields.game,
      winner: fields.winner,
    }
  }

  toJSON(): CrankLottoGameWinnerEventJSON {
    return {
      user: this.user.toString(),
      game: this.game.toString(),
      winner: this.winner.toString(),
    }
  }

  static fromJSON(
    obj: CrankLottoGameWinnerEventJSON
  ): CrankLottoGameWinnerEvent {
    return new CrankLottoGameWinnerEvent({
      user: new PublicKey(obj.user),
      game: new PublicKey(obj.game),
      winner: new PublicKey(obj.winner),
    })
  }

  toEncodable() {
    return CrankLottoGameWinnerEvent.toEncodable(this)
  }
}
