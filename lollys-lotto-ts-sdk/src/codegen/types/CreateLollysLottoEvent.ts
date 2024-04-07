import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CreateLollysLottoEventFields {
  authority: PublicKey
  lollysLotto: PublicKey
  lottoGameCount: BN
}

export interface CreateLollysLottoEventJSON {
  authority: string
  lollysLotto: string
  lottoGameCount: string
}

/** Event emitted when a user creates a lolly lotto. */
export class CreateLollysLottoEvent {
  readonly authority: PublicKey
  readonly lollysLotto: PublicKey
  readonly lottoGameCount: BN

  constructor(fields: CreateLollysLottoEventFields) {
    this.authority = fields.authority
    this.lollysLotto = fields.lollysLotto
    this.lottoGameCount = fields.lottoGameCount
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("authority"),
        borsh.publicKey("lollysLotto"),
        borsh.u64("lottoGameCount"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CreateLollysLottoEvent({
      authority: obj.authority,
      lollysLotto: obj.lollysLotto,
      lottoGameCount: obj.lottoGameCount,
    })
  }

  static toEncodable(fields: CreateLollysLottoEventFields) {
    return {
      authority: fields.authority,
      lollysLotto: fields.lollysLotto,
      lottoGameCount: fields.lottoGameCount,
    }
  }

  toJSON(): CreateLollysLottoEventJSON {
    return {
      authority: this.authority.toString(),
      lollysLotto: this.lollysLotto.toString(),
      lottoGameCount: this.lottoGameCount.toString(),
    }
  }

  static fromJSON(obj: CreateLollysLottoEventJSON): CreateLollysLottoEvent {
    return new CreateLollysLottoEvent({
      authority: new PublicKey(obj.authority),
      lollysLotto: new PublicKey(obj.lollysLotto),
      lottoGameCount: new BN(obj.lottoGameCount),
    })
  }

  toEncodable() {
    return CreateLollysLottoEvent.toEncodable(this)
  }
}
