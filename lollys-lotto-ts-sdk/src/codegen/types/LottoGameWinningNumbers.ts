import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface LottoGameWinningNumbersFields {
  number1: number
  number2: number
  number3: number
  number4: number
  number5: number
  jackpotNumber: number
  switchboardRandomNumbersUpdated: types.WinningNumberUpdateStateKind
  winningAmountDisbursed: types.WinningAmountDisbursedStateKind
}

export interface LottoGameWinningNumbersJSON {
  number1: number
  number2: number
  number3: number
  number4: number
  number5: number
  jackpotNumber: number
  switchboardRandomNumbersUpdated: types.WinningNumberUpdateStateJSON
  winningAmountDisbursed: types.WinningAmountDisbursedStateJSON
}

/** Compile-time size check. */
export class LottoGameWinningNumbers {
  readonly number1: number
  readonly number2: number
  readonly number3: number
  readonly number4: number
  readonly number5: number
  readonly jackpotNumber: number
  readonly switchboardRandomNumbersUpdated: types.WinningNumberUpdateStateKind
  readonly winningAmountDisbursed: types.WinningAmountDisbursedStateKind

  constructor(fields: LottoGameWinningNumbersFields) {
    this.number1 = fields.number1
    this.number2 = fields.number2
    this.number3 = fields.number3
    this.number4 = fields.number4
    this.number5 = fields.number5
    this.jackpotNumber = fields.jackpotNumber
    this.switchboardRandomNumbersUpdated =
      fields.switchboardRandomNumbersUpdated
    this.winningAmountDisbursed = fields.winningAmountDisbursed
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u8("number1"),
        borsh.u8("number2"),
        borsh.u8("number3"),
        borsh.u8("number4"),
        borsh.u8("number5"),
        borsh.u8("jackpotNumber"),
        types.WinningNumberUpdateState.layout(
          "switchboardRandomNumbersUpdated"
        ),
        types.WinningAmountDisbursedState.layout("winningAmountDisbursed"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new LottoGameWinningNumbers({
      number1: obj.number1,
      number2: obj.number2,
      number3: obj.number3,
      number4: obj.number4,
      number5: obj.number5,
      jackpotNumber: obj.jackpotNumber,
      switchboardRandomNumbersUpdated:
        types.WinningNumberUpdateState.fromDecoded(
          obj.switchboardRandomNumbersUpdated
        ),
      winningAmountDisbursed: types.WinningAmountDisbursedState.fromDecoded(
        obj.winningAmountDisbursed
      ),
    })
  }

  static toEncodable(fields: LottoGameWinningNumbersFields) {
    return {
      number1: fields.number1,
      number2: fields.number2,
      number3: fields.number3,
      number4: fields.number4,
      number5: fields.number5,
      jackpotNumber: fields.jackpotNumber,
      switchboardRandomNumbersUpdated:
        fields.switchboardRandomNumbersUpdated.toEncodable(),
      winningAmountDisbursed: fields.winningAmountDisbursed.toEncodable(),
    }
  }

  toJSON(): LottoGameWinningNumbersJSON {
    return {
      number1: this.number1,
      number2: this.number2,
      number3: this.number3,
      number4: this.number4,
      number5: this.number5,
      jackpotNumber: this.jackpotNumber,
      switchboardRandomNumbersUpdated:
        this.switchboardRandomNumbersUpdated.toJSON(),
      winningAmountDisbursed: this.winningAmountDisbursed.toJSON(),
    }
  }

  static fromJSON(obj: LottoGameWinningNumbersJSON): LottoGameWinningNumbers {
    return new LottoGameWinningNumbers({
      number1: obj.number1,
      number2: obj.number2,
      number3: obj.number3,
      number4: obj.number4,
      number5: obj.number5,
      jackpotNumber: obj.jackpotNumber,
      switchboardRandomNumbersUpdated: types.WinningNumberUpdateState.fromJSON(
        obj.switchboardRandomNumbersUpdated
      ),
      winningAmountDisbursed: types.WinningAmountDisbursedState.fromJSON(
        obj.winningAmountDisbursed
      ),
    })
  }

  toEncodable() {
    return LottoGameWinningNumbers.toEncodable(this)
  }
}
