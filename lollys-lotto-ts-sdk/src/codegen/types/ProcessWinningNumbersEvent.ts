import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface ProcessWinningNumbersEventFields {
  lottoGame: PublicKey
  round: BN
  randomness: Array<number>
  winningNumbers: Array<number>
  winningNumbersUpdatedIndex: Array<BN>
}

export interface ProcessWinningNumbersEventJSON {
  lottoGame: string
  round: string
  randomness: Array<number>
  winningNumbers: Array<number>
  winningNumbersUpdatedIndex: Array<string>
}

/**
 * Event emitted when a user consumes randomness.
 * This is a placeholder event, and will be replaced with a more
 * meaningful event in the future.
 */
export class ProcessWinningNumbersEvent {
  readonly lottoGame: PublicKey
  readonly round: BN
  readonly randomness: Array<number>
  readonly winningNumbers: Array<number>
  readonly winningNumbersUpdatedIndex: Array<BN>

  constructor(fields: ProcessWinningNumbersEventFields) {
    this.lottoGame = fields.lottoGame
    this.round = fields.round
    this.randomness = fields.randomness
    this.winningNumbers = fields.winningNumbers
    this.winningNumbersUpdatedIndex = fields.winningNumbersUpdatedIndex
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("lottoGame"),
        borsh.u64("round"),
        borsh.array(borsh.u8(), 32, "randomness"),
        borsh.array(borsh.u8(), 6, "winningNumbers"),
        borsh.array(borsh.i64(), 4, "winningNumbersUpdatedIndex"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new ProcessWinningNumbersEvent({
      lottoGame: obj.lottoGame,
      round: obj.round,
      randomness: obj.randomness,
      winningNumbers: obj.winningNumbers,
      winningNumbersUpdatedIndex: obj.winningNumbersUpdatedIndex,
    })
  }

  static toEncodable(fields: ProcessWinningNumbersEventFields) {
    return {
      lottoGame: fields.lottoGame,
      round: fields.round,
      randomness: fields.randomness,
      winningNumbers: fields.winningNumbers,
      winningNumbersUpdatedIndex: fields.winningNumbersUpdatedIndex,
    }
  }

  toJSON(): ProcessWinningNumbersEventJSON {
    return {
      lottoGame: this.lottoGame.toString(),
      round: this.round.toString(),
      randomness: this.randomness,
      winningNumbers: this.winningNumbers,
      winningNumbersUpdatedIndex: this.winningNumbersUpdatedIndex.map((item) =>
        item.toString()
      ),
    }
  }

  static fromJSON(
    obj: ProcessWinningNumbersEventJSON
  ): ProcessWinningNumbersEvent {
    return new ProcessWinningNumbersEvent({
      lottoGame: new PublicKey(obj.lottoGame),
      round: new BN(obj.round),
      randomness: obj.randomness,
      winningNumbers: obj.winningNumbers,
      winningNumbersUpdatedIndex: obj.winningNumbersUpdatedIndex.map(
        (item) => new BN(item)
      ),
    })
  }

  toEncodable() {
    return ProcessWinningNumbersEvent.toEncodable(this)
  }
}
