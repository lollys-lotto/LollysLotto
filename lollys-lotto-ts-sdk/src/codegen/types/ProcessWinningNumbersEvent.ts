import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface ProcessWinningNumbersEventFields {
  round: BN
  randomness: Uint8Array
}

export interface ProcessWinningNumbersEventJSON {
  round: string
  randomness: Array<number>
}

/**
 * Event emitted when a user consumes randomness.
 * This is a placeholder event, and will be replaced with a more
 * meaningful event in the future.
 */
export class ProcessWinningNumbersEvent {
  readonly round: BN
  readonly randomness: Uint8Array

  constructor(fields: ProcessWinningNumbersEventFields) {
    this.round = fields.round
    this.randomness = fields.randomness
  }

  static layout(property?: string) {
    return borsh.struct(
      [borsh.u64("round"), borsh.vecU8("randomness")],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new ProcessWinningNumbersEvent({
      round: obj.round,
      randomness: new Uint8Array(
        obj.randomness.buffer,
        obj.randomness.byteOffset,
        obj.randomness.length
      ),
    })
  }

  static toEncodable(fields: ProcessWinningNumbersEventFields) {
    return {
      round: fields.round,
      randomness: Buffer.from(
        fields.randomness.buffer,
        fields.randomness.byteOffset,
        fields.randomness.length
      ),
    }
  }

  toJSON(): ProcessWinningNumbersEventJSON {
    return {
      round: this.round.toString(),
      randomness: Array.from(this.randomness.values()),
    }
  }

  static fromJSON(
    obj: ProcessWinningNumbersEventJSON
  ): ProcessWinningNumbersEvent {
    return new ProcessWinningNumbersEvent({
      round: new BN(obj.round),
      randomness: Uint8Array.from(obj.randomness),
    })
  }

  toEncodable() {
    return ProcessWinningNumbersEvent.toEncodable(this)
  }
}
