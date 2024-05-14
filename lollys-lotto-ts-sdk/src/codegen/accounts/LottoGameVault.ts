import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface LottoGameVaultFields {}

export interface LottoGameVaultJSON {}

/**
 * Does not need to be initialized, only to act as a signer.
 * This signer owns the USDC token account where ticket sales in USDC is stored.
 */
export class LottoGameVault {
  static readonly discriminator = Buffer.from([
    87, 181, 46, 213, 76, 162, 154, 135,
  ])

  static readonly layout = borsh.struct([])

  constructor(fields: LottoGameVaultFields) {}

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<LottoGameVault | null> {
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
  ): Promise<Array<LottoGameVault | null>> {
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

  static decode(data: Buffer): LottoGameVault {
    if (!data.slice(0, 8).equals(LottoGameVault.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = LottoGameVault.layout.decode(data.slice(8))

    return new LottoGameVault({})
  }

  toJSON(): LottoGameVaultJSON {
    return {}
  }

  static fromJSON(obj: LottoGameVaultJSON): LottoGameVault {
    return new LottoGameVault({})
  }
}
