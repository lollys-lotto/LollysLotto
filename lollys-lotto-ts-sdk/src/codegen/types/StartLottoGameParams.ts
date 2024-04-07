import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface StartLottoGameParamsFields {
  round: BN
  ticketPrice: BN
  gameDuration: BN
  roundName: string
}

export interface StartLottoGameParamsJSON {
  round: string
  ticketPrice: string
  gameDuration: string
  roundName: string
}

export class StartLottoGameParams {
  readonly round: BN
  readonly ticketPrice: BN
  readonly gameDuration: BN
  readonly roundName: string

  constructor(fields: StartLottoGameParamsFields) {
    this.round = fields.round
    this.ticketPrice = fields.ticketPrice
    this.gameDuration = fields.gameDuration
    this.roundName = fields.roundName
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u64("round"),
        borsh.u64("ticketPrice"),
        borsh.u64("gameDuration"),
        borsh.str("roundName"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new StartLottoGameParams({
      round: obj.round,
      ticketPrice: obj.ticketPrice,
      gameDuration: obj.gameDuration,
      roundName: obj.roundName,
    })
  }

  static toEncodable(fields: StartLottoGameParamsFields) {
    return {
      round: fields.round,
      ticketPrice: fields.ticketPrice,
      gameDuration: fields.gameDuration,
      roundName: fields.roundName,
    }
  }

  toJSON(): StartLottoGameParamsJSON {
    return {
      round: this.round.toString(),
      ticketPrice: this.ticketPrice.toString(),
      gameDuration: this.gameDuration.toString(),
      roundName: this.roundName,
    }
  }

  static fromJSON(obj: StartLottoGameParamsJSON): StartLottoGameParams {
    return new StartLottoGameParams({
      round: new BN(obj.round),
      ticketPrice: new BN(obj.ticketPrice),
      gameDuration: new BN(obj.gameDuration),
      roundName: obj.roundName,
    })
  }

  toEncodable() {
    return StartLottoGameParams.toEncodable(this)
  }
}
