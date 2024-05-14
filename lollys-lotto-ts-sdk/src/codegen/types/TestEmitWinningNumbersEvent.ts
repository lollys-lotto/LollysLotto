import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface TestEmitWinningNumbersEventFields {
  round: BN
  randomness: Uint8Array
}

export interface TestEmitWinningNumbersEventJSON {
  round: string
  randomness: Array<number>
}

/** Test Event emitted when a user requests randomness. */
export class TestEmitWinningNumbersEvent {
  readonly round: BN
  readonly randomness: Uint8Array

  constructor(fields: TestEmitWinningNumbersEventFields) {
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
    return new TestEmitWinningNumbersEvent({
      round: obj.round,
      randomness: new Uint8Array(
        obj.randomness.buffer,
        obj.randomness.byteOffset,
        obj.randomness.length
      ),
    })
  }

  static toEncodable(fields: TestEmitWinningNumbersEventFields) {
    return {
      round: fields.round,
      randomness: Buffer.from(
        fields.randomness.buffer,
        fields.randomness.byteOffset,
        fields.randomness.length
      ),
    }
  }

  toJSON(): TestEmitWinningNumbersEventJSON {
    return {
      round: this.round.toString(),
      randomness: Array.from(this.randomness.values()),
    }
  }

  static fromJSON(
    obj: TestEmitWinningNumbersEventJSON
  ): TestEmitWinningNumbersEvent {
    return new TestEmitWinningNumbersEvent({
      round: new BN(obj.round),
      randomness: Uint8Array.from(obj.randomness),
    })
  }

  toEncodable() {
    return TestEmitWinningNumbersEvent.toEncodable(this)
  }
}
