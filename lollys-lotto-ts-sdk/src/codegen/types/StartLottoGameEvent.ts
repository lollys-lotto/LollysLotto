import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface StartLottoGameEventFields {
  lottoGameVersion: types.LottoGameVersionKind
  round: BN
  roundName: string
  gameDuration: BN
  authority: PublicKey
  lottoGamePubkey: PublicKey
  lottoGameVault: PublicKey
  lottoGameMint: PublicKey
  startDate: BN
  endDate: BN
  ticketPrice: BN
  state: types.LottoGameStateKind
  lottoGameCount: BN
}

export interface StartLottoGameEventJSON {
  lottoGameVersion: types.LottoGameVersionJSON
  round: string
  roundName: string
  gameDuration: string
  authority: string
  lottoGamePubkey: string
  lottoGameVault: string
  lottoGameMint: string
  startDate: string
  endDate: string
  ticketPrice: string
  state: types.LottoGameStateJSON
  lottoGameCount: string
}

/** Event emitted when a user starts a lotto game. */
export class StartLottoGameEvent {
  readonly lottoGameVersion: types.LottoGameVersionKind
  readonly round: BN
  readonly roundName: string
  readonly gameDuration: BN
  readonly authority: PublicKey
  readonly lottoGamePubkey: PublicKey
  readonly lottoGameVault: PublicKey
  readonly lottoGameMint: PublicKey
  readonly startDate: BN
  readonly endDate: BN
  readonly ticketPrice: BN
  readonly state: types.LottoGameStateKind
  readonly lottoGameCount: BN

  constructor(fields: StartLottoGameEventFields) {
    this.lottoGameVersion = fields.lottoGameVersion
    this.round = fields.round
    this.roundName = fields.roundName
    this.gameDuration = fields.gameDuration
    this.authority = fields.authority
    this.lottoGamePubkey = fields.lottoGamePubkey
    this.lottoGameVault = fields.lottoGameVault
    this.lottoGameMint = fields.lottoGameMint
    this.startDate = fields.startDate
    this.endDate = fields.endDate
    this.ticketPrice = fields.ticketPrice
    this.state = fields.state
    this.lottoGameCount = fields.lottoGameCount
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        types.LottoGameVersion.layout("lottoGameVersion"),
        borsh.u64("round"),
        borsh.str("roundName"),
        borsh.u64("gameDuration"),
        borsh.publicKey("authority"),
        borsh.publicKey("lottoGamePubkey"),
        borsh.publicKey("lottoGameVault"),
        borsh.publicKey("lottoGameMint"),
        borsh.i64("startDate"),
        borsh.i64("endDate"),
        borsh.u64("ticketPrice"),
        types.LottoGameState.layout("state"),
        borsh.u64("lottoGameCount"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new StartLottoGameEvent({
      lottoGameVersion: types.LottoGameVersion.fromDecoded(
        obj.lottoGameVersion
      ),
      round: obj.round,
      roundName: obj.roundName,
      gameDuration: obj.gameDuration,
      authority: obj.authority,
      lottoGamePubkey: obj.lottoGamePubkey,
      lottoGameVault: obj.lottoGameVault,
      lottoGameMint: obj.lottoGameMint,
      startDate: obj.startDate,
      endDate: obj.endDate,
      ticketPrice: obj.ticketPrice,
      state: types.LottoGameState.fromDecoded(obj.state),
      lottoGameCount: obj.lottoGameCount,
    })
  }

  static toEncodable(fields: StartLottoGameEventFields) {
    return {
      lottoGameVersion: fields.lottoGameVersion.toEncodable(),
      round: fields.round,
      roundName: fields.roundName,
      gameDuration: fields.gameDuration,
      authority: fields.authority,
      lottoGamePubkey: fields.lottoGamePubkey,
      lottoGameVault: fields.lottoGameVault,
      lottoGameMint: fields.lottoGameMint,
      startDate: fields.startDate,
      endDate: fields.endDate,
      ticketPrice: fields.ticketPrice,
      state: fields.state.toEncodable(),
      lottoGameCount: fields.lottoGameCount,
    }
  }

  toJSON(): StartLottoGameEventJSON {
    return {
      lottoGameVersion: this.lottoGameVersion.toJSON(),
      round: this.round.toString(),
      roundName: this.roundName,
      gameDuration: this.gameDuration.toString(),
      authority: this.authority.toString(),
      lottoGamePubkey: this.lottoGamePubkey.toString(),
      lottoGameVault: this.lottoGameVault.toString(),
      lottoGameMint: this.lottoGameMint.toString(),
      startDate: this.startDate.toString(),
      endDate: this.endDate.toString(),
      ticketPrice: this.ticketPrice.toString(),
      state: this.state.toJSON(),
      lottoGameCount: this.lottoGameCount.toString(),
    }
  }

  static fromJSON(obj: StartLottoGameEventJSON): StartLottoGameEvent {
    return new StartLottoGameEvent({
      lottoGameVersion: types.LottoGameVersion.fromJSON(obj.lottoGameVersion),
      round: new BN(obj.round),
      roundName: obj.roundName,
      gameDuration: new BN(obj.gameDuration),
      authority: new PublicKey(obj.authority),
      lottoGamePubkey: new PublicKey(obj.lottoGamePubkey),
      lottoGameVault: new PublicKey(obj.lottoGameVault),
      lottoGameMint: new PublicKey(obj.lottoGameMint),
      startDate: new BN(obj.startDate),
      endDate: new BN(obj.endDate),
      ticketPrice: new BN(obj.ticketPrice),
      state: types.LottoGameState.fromJSON(obj.state),
      lottoGameCount: new BN(obj.lottoGameCount),
    })
  }

  toEncodable() {
    return StartLottoGameEvent.toEncodable(this)
  }
}
