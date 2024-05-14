import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface LollysLottoFields {
  bump: number
  lottoGameCount: BN
  authority: PublicKey
}

export interface LollysLottoJSON {
  bump: number
  lottoGameCount: string
  authority: string
}

export class LollysLotto {
  readonly bump: number
  readonly lottoGameCount: BN
  readonly authority: PublicKey

  static readonly discriminator = Buffer.from([
    34, 54, 162, 60, 253, 27, 228, 195,
  ])

  static readonly layout = borsh.struct([
    borsh.u8("bump"),
    borsh.u64("lottoGameCount"),
    borsh.publicKey("authority"),
  ])

  constructor(fields: LollysLottoFields) {
    this.bump = fields.bump
    this.lottoGameCount = fields.lottoGameCount
    this.authority = fields.authority
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<LollysLotto | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(programId)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[],
    programId: PublicKey = PROGRAM_ID
  ): Promise<Array<LollysLotto | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(programId)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): LollysLotto {
    if (!data.slice(0, 8).equals(LollysLotto.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = LollysLotto.layout.decode(data.slice(8))

    return new LollysLotto({
      bump: dec.bump,
      lottoGameCount: dec.lottoGameCount,
      authority: dec.authority,
    })
  }

  toJSON(): LollysLottoJSON {
    return {
      bump: this.bump,
      lottoGameCount: this.lottoGameCount.toString(),
      authority: this.authority.toString(),
    }
  }

  static fromJSON(obj: LollysLottoJSON): LollysLotto {
    return new LollysLotto({
      bump: obj.bump,
      lottoGameCount: new BN(obj.lottoGameCount),
      authority: new PublicKey(obj.authority),
    })
  }
}
