import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface StartLottoGameEventFields {
  round: BN
  roundName: string
  gameDuration: BN
  lottoGamePubkey: PublicKey
  startDate: BN
  endDate: BN
  ticketPrice: BN
  state: types.LottoGameStateKind
}

export interface StartLottoGameEventJSON {
  round: string
  roundName: string
  gameDuration: string
  lottoGamePubkey: string
  startDate: string
  endDate: string
  ticketPrice: string
  state: types.LottoGameStateJSON
}

/** Event emitted when a user starts a lotto game. */
export class StartLottoGameEvent {
  readonly round: BN
  readonly roundName: string
  readonly gameDuration: BN
  readonly lottoGamePubkey: PublicKey
  readonly startDate: BN
  readonly endDate: BN
  readonly ticketPrice: BN
  readonly state: types.LottoGameStateKind

  constructor(fields: StartLottoGameEventFields) {
    this.round = fields.round
    this.roundName = fields.roundName
    this.gameDuration = fields.gameDuration
    this.lottoGamePubkey = fields.lottoGamePubkey
    this.startDate = fields.startDate
    this.endDate = fields.endDate
    this.ticketPrice = fields.ticketPrice
    this.state = fields.state
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u64("round"),
        borsh.str("roundName"),
        borsh.u64("gameDuration"),
        borsh.publicKey("lottoGamePubkey"),
        borsh.i64("startDate"),
        borsh.i64("endDate"),
        borsh.u64("ticketPrice"),
        types.LottoGameState.layout("state"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new StartLottoGameEvent({
      round: obj.round,
      roundName: obj.roundName,
      gameDuration: obj.gameDuration,
      lottoGamePubkey: obj.lottoGamePubkey,
      startDate: obj.startDate,
      endDate: obj.endDate,
      ticketPrice: obj.ticketPrice,
      state: types.LottoGameState.fromDecoded(obj.state),
    })
  }

  static toEncodable(fields: StartLottoGameEventFields) {
    return {
      round: fields.round,
      roundName: fields.roundName,
      gameDuration: fields.gameDuration,
      lottoGamePubkey: fields.lottoGamePubkey,
      startDate: fields.startDate,
      endDate: fields.endDate,
      ticketPrice: fields.ticketPrice,
      state: fields.state.toEncodable(),
    }
  }

  toJSON(): StartLottoGameEventJSON {
    return {
      round: this.round.toString(),
      roundName: this.roundName,
      gameDuration: this.gameDuration.toString(),
      lottoGamePubkey: this.lottoGamePubkey.toString(),
      startDate: this.startDate.toString(),
      endDate: this.endDate.toString(),
      ticketPrice: this.ticketPrice.toString(),
      state: this.state.toJSON(),
    }
  }

  static fromJSON(obj: StartLottoGameEventJSON): StartLottoGameEvent {
    return new StartLottoGameEvent({
      round: new BN(obj.round),
      roundName: obj.roundName,
      gameDuration: new BN(obj.gameDuration),
      lottoGamePubkey: new PublicKey(obj.lottoGamePubkey),
      startDate: new BN(obj.startDate),
      endDate: new BN(obj.endDate),
      ticketPrice: new BN(obj.ticketPrice),
      state: types.LottoGameState.fromJSON(obj.state),
    })
  }

  toEncodable() {
    return StartLottoGameEvent.toEncodable(this)
  }
}
