import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface UserMetadataFields {
  bump: number
  user: PublicKey
  createdTimestamp: BN
  tier: types.UserTierKind
  totalTicketsPurchased: BN
  totalAmountWon: BN
  totalAmountClaimed: BN
  lastClaimedAt: BN
  referralCount: BN
  referralRevenue: BN
  claimTickets: Array<types.ClaimTicketFields>
}

export interface UserMetadataJSON {
  bump: number
  user: string
  createdTimestamp: string
  tier: types.UserTierJSON
  totalTicketsPurchased: string
  totalAmountWon: string
  totalAmountClaimed: string
  lastClaimedAt: string
  referralCount: string
  referralRevenue: string
  claimTickets: Array<types.ClaimTicketJSON>
}

export class UserMetadata {
  readonly bump: number
  readonly user: PublicKey
  readonly createdTimestamp: BN
  readonly tier: types.UserTierKind
  readonly totalTicketsPurchased: BN
  readonly totalAmountWon: BN
  readonly totalAmountClaimed: BN
  readonly lastClaimedAt: BN
  readonly referralCount: BN
  readonly referralRevenue: BN
  readonly claimTickets: Array<types.ClaimTicket>

  static readonly discriminator = Buffer.from([
    157, 214, 220, 235, 98, 135, 171, 28,
  ])

  static readonly layout = borsh.struct([
    borsh.u8("bump"),
    borsh.publicKey("user"),
    borsh.i64("createdTimestamp"),
    types.UserTier.layout("tier"),
    borsh.u64("totalTicketsPurchased"),
    borsh.u64("totalAmountWon"),
    borsh.u64("totalAmountClaimed"),
    borsh.i64("lastClaimedAt"),
    borsh.u64("referralCount"),
    borsh.u64("referralRevenue"),
    borsh.array(types.ClaimTicket.layout(), 64, "claimTickets"),
  ])

  constructor(fields: UserMetadataFields) {
    this.bump = fields.bump
    this.user = fields.user
    this.createdTimestamp = fields.createdTimestamp
    this.tier = fields.tier
    this.totalTicketsPurchased = fields.totalTicketsPurchased
    this.totalAmountWon = fields.totalAmountWon
    this.totalAmountClaimed = fields.totalAmountClaimed
    this.lastClaimedAt = fields.lastClaimedAt
    this.referralCount = fields.referralCount
    this.referralRevenue = fields.referralRevenue
    this.claimTickets = fields.claimTickets.map(
      (item) => new types.ClaimTicket({ ...item })
    )
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<UserMetadata | null> {
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
  ): Promise<Array<UserMetadata | null>> {
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

  static decode(data: Buffer): UserMetadata {
    if (!data.subarray(0, 8).equals(UserMetadata.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = UserMetadata.layout.decode(data.subarray(8))

    return new UserMetadata({
      bump: dec.bump,
      user: dec.user,
      createdTimestamp: dec.createdTimestamp,
      tier: types.UserTier.fromDecoded(dec.tier),
      totalTicketsPurchased: dec.totalTicketsPurchased,
      totalAmountWon: dec.totalAmountWon,
      totalAmountClaimed: dec.totalAmountClaimed,
      lastClaimedAt: dec.lastClaimedAt,
      referralCount: dec.referralCount,
      referralRevenue: dec.referralRevenue,
      claimTickets: dec.claimTickets.map(
        (
          item: any /* eslint-disable-line @typescript-eslint/no-explicit-any */
        ) => types.ClaimTicket.fromDecoded(item)
      ),
    })
  }

  toJSON(): UserMetadataJSON {
    return {
      bump: this.bump,
      user: this.user.toString(),
      createdTimestamp: this.createdTimestamp.toString(),
      tier: this.tier.toJSON(),
      totalTicketsPurchased: this.totalTicketsPurchased.toString(),
      totalAmountWon: this.totalAmountWon.toString(),
      totalAmountClaimed: this.totalAmountClaimed.toString(),
      lastClaimedAt: this.lastClaimedAt.toString(),
      referralCount: this.referralCount.toString(),
      referralRevenue: this.referralRevenue.toString(),
      claimTickets: this.claimTickets.map((item) => item.toJSON()),
    }
  }

  static fromJSON(obj: UserMetadataJSON): UserMetadata {
    return new UserMetadata({
      bump: obj.bump,
      user: new PublicKey(obj.user),
      createdTimestamp: new BN(obj.createdTimestamp),
      tier: types.UserTier.fromJSON(obj.tier),
      totalTicketsPurchased: new BN(obj.totalTicketsPurchased),
      totalAmountWon: new BN(obj.totalAmountWon),
      totalAmountClaimed: new BN(obj.totalAmountClaimed),
      lastClaimedAt: new BN(obj.lastClaimedAt),
      referralCount: new BN(obj.referralCount),
      referralRevenue: new BN(obj.referralRevenue),
      claimTickets: obj.claimTickets.map((item) =>
        types.ClaimTicket.fromJSON(item)
      ),
    })
  }
}
