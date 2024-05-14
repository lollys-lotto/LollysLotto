import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CrankLottoGameClosedEventFields {
  lottoGame: PublicKey
  round: BN
  ticketPrice: BN
  gameDuration: BN
}

export interface CrankLottoGameClosedEventJSON {
  lottoGame: string
  round: string
  ticketPrice: string
  gameDuration: string
}

/** Event emitted when a user updates a lotto game. */
export class CrankLottoGameClosedEvent {
  readonly lottoGame: PublicKey
  readonly round: BN
  readonly ticketPrice: BN
  readonly gameDuration: BN

  constructor(fields: CrankLottoGameClosedEventFields) {
    this.lottoGame = fields.lottoGame
    this.round = fields.round
    this.ticketPrice = fields.ticketPrice
    this.gameDuration = fields.gameDuration
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("lottoGame"),
        borsh.u64("round"),
        borsh.u64("ticketPrice"),
        borsh.u64("gameDuration"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CrankLottoGameClosedEvent({
      lottoGame: obj.lottoGame,
      round: obj.round,
      ticketPrice: obj.ticketPrice,
      gameDuration: obj.gameDuration,
    })
  }

  static toEncodable(fields: CrankLottoGameClosedEventFields) {
    return {
      lottoGame: fields.lottoGame,
      round: fields.round,
      ticketPrice: fields.ticketPrice,
      gameDuration: fields.gameDuration,
    }
  }

  toJSON(): CrankLottoGameClosedEventJSON {
    return {
      lottoGame: this.lottoGame.toString(),
      round: this.round.toString(),
      ticketPrice: this.ticketPrice.toString(),
      gameDuration: this.gameDuration.toString(),
    }
  }

  static fromJSON(
    obj: CrankLottoGameClosedEventJSON
  ): CrankLottoGameClosedEvent {
    return new CrankLottoGameClosedEvent({
      lottoGame: new PublicKey(obj.lottoGame),
      round: new BN(obj.round),
      ticketPrice: new BN(obj.ticketPrice),
      gameDuration: new BN(obj.gameDuration),
    })
  }

  toEncodable() {
    return CrankLottoGameClosedEvent.toEncodable(this)
  }
}
