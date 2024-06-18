import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface DuplicateWinningNumbersEventFields {
  lottoGame: PublicKey
  round: BN
  randomness: Array<number>
  duplicateNumbers: Array<number>
  duplicateNumberDetectedIndex: Array<BN>
}

export interface DuplicateWinningNumbersEventJSON {
  lottoGame: string
  round: string
  randomness: Array<number>
  duplicateNumbers: Array<number>
  duplicateNumberDetectedIndex: Array<string>
}

/** Event emitted when duplicate winning numbers are detected. */
export class DuplicateWinningNumbersEvent {
  readonly lottoGame: PublicKey
  readonly round: BN
  readonly randomness: Array<number>
  readonly duplicateNumbers: Array<number>
  readonly duplicateNumberDetectedIndex: Array<BN>

  constructor(fields: DuplicateWinningNumbersEventFields) {
    this.lottoGame = fields.lottoGame
    this.round = fields.round
    this.randomness = fields.randomness
    this.duplicateNumbers = fields.duplicateNumbers
    this.duplicateNumberDetectedIndex = fields.duplicateNumberDetectedIndex
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("lottoGame"),
        borsh.u64("round"),
        borsh.array(borsh.u8(), 32, "randomness"),
        borsh.array(borsh.u8(), 6, "duplicateNumbers"),
        borsh.array(borsh.i64(), 4, "duplicateNumberDetectedIndex"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new DuplicateWinningNumbersEvent({
      lottoGame: obj.lottoGame,
      round: obj.round,
      randomness: obj.randomness,
      duplicateNumbers: obj.duplicateNumbers,
      duplicateNumberDetectedIndex: obj.duplicateNumberDetectedIndex,
    })
  }

  static toEncodable(fields: DuplicateWinningNumbersEventFields) {
    return {
      lottoGame: fields.lottoGame,
      round: fields.round,
      randomness: fields.randomness,
      duplicateNumbers: fields.duplicateNumbers,
      duplicateNumberDetectedIndex: fields.duplicateNumberDetectedIndex,
    }
  }

  toJSON(): DuplicateWinningNumbersEventJSON {
    return {
      lottoGame: this.lottoGame.toString(),
      round: this.round.toString(),
      randomness: this.randomness,
      duplicateNumbers: this.duplicateNumbers,
      duplicateNumberDetectedIndex: this.duplicateNumberDetectedIndex.map(
        (item) => item.toString()
      ),
    }
  }

  static fromJSON(
    obj: DuplicateWinningNumbersEventJSON
  ): DuplicateWinningNumbersEvent {
    return new DuplicateWinningNumbersEvent({
      lottoGame: new PublicKey(obj.lottoGame),
      round: new BN(obj.round),
      randomness: obj.randomness,
      duplicateNumbers: obj.duplicateNumbers,
      duplicateNumberDetectedIndex: obj.duplicateNumberDetectedIndex.map(
        (item) => new BN(item)
      ),
    })
  }

  toEncodable() {
    return DuplicateWinningNumbersEvent.toEncodable(this)
  }
}
