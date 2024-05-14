import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface LollyBurnStateFields {
  bump: number
  totalLollyBurnt: BN
  authority: PublicKey
}

export interface LollyBurnStateJSON {
  bump: number
  totalLollyBurnt: string
  authority: string
}

export class LollyBurnState {
  readonly bump: number
  readonly totalLollyBurnt: BN
  readonly authority: PublicKey

  static readonly discriminator = Buffer.from([
    72, 125, 145, 166, 128, 37, 245, 220,
  ])

  static readonly layout = borsh.struct([
    borsh.u8("bump"),
    borsh.u64("totalLollyBurnt"),
    borsh.publicKey("authority"),
  ])

  constructor(fields: LollyBurnStateFields) {
    this.bump = fields.bump
    this.totalLollyBurnt = fields.totalLollyBurnt
    this.authority = fields.authority
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<LollyBurnState | null> {
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
  ): Promise<Array<LollyBurnState | null>> {
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

  static decode(data: Buffer): LollyBurnState {
    if (!data.slice(0, 8).equals(LollyBurnState.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = LollyBurnState.layout.decode(data.slice(8))

    return new LollyBurnState({
      bump: dec.bump,
      totalLollyBurnt: dec.totalLollyBurnt,
      authority: dec.authority,
    })
  }

  toJSON(): LollyBurnStateJSON {
    return {
      bump: this.bump,
      totalLollyBurnt: this.totalLollyBurnt.toString(),
      authority: this.authority.toString(),
    }
  }

  static fromJSON(obj: LollyBurnStateJSON): LollyBurnState {
    return new LollyBurnState({
      bump: obj.bump,
      totalLollyBurnt: new BN(obj.totalLollyBurnt),
      authority: new PublicKey(obj.authority),
    })
  }
}
