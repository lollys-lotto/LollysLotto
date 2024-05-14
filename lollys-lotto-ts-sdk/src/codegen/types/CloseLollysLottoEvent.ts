import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CloseLollysLottoEventFields {
  lollysLotto: PublicKey
}

export interface CloseLollysLottoEventJSON {
  lollysLotto: string
}

export class CloseLollysLottoEvent {
  readonly lollysLotto: PublicKey

  constructor(fields: CloseLollysLottoEventFields) {
    this.lollysLotto = fields.lollysLotto
  }

  static layout(property?: string) {
    return borsh.struct([borsh.publicKey("lollysLotto")], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CloseLollysLottoEvent({
      lollysLotto: obj.lollysLotto,
    })
  }

  static toEncodable(fields: CloseLollysLottoEventFields) {
    return {
      lollysLotto: fields.lollysLotto,
    }
  }

  toJSON(): CloseLollysLottoEventJSON {
    return {
      lollysLotto: this.lollysLotto.toString(),
    }
  }

  static fromJSON(obj: CloseLollysLottoEventJSON): CloseLollysLottoEvent {
    return new CloseLollysLottoEvent({
      lollysLotto: new PublicKey(obj.lollysLotto),
    })
  }

  toEncodable() {
    return CloseLollysLottoEvent.toEncodable(this)
  }
}
