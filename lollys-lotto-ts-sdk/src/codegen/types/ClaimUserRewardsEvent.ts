import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface ClaimUserRewardsEventFields {
  user: PublicKey
  userMetadata: PublicKey
  userRewardsVault: PublicKey
  amountToBeClaimed: BN
  totalAmountClaimed: BN
}

export interface ClaimUserRewardsEventJSON {
  user: string
  userMetadata: string
  userRewardsVault: string
  amountToBeClaimed: string
  totalAmountClaimed: string
}

export class ClaimUserRewardsEvent {
  readonly user: PublicKey
  readonly userMetadata: PublicKey
  readonly userRewardsVault: PublicKey
  readonly amountToBeClaimed: BN
  readonly totalAmountClaimed: BN

  constructor(fields: ClaimUserRewardsEventFields) {
    this.user = fields.user
    this.userMetadata = fields.userMetadata
    this.userRewardsVault = fields.userRewardsVault
    this.amountToBeClaimed = fields.amountToBeClaimed
    this.totalAmountClaimed = fields.totalAmountClaimed
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("user"),
        borsh.publicKey("userMetadata"),
        borsh.publicKey("userRewardsVault"),
        borsh.u64("amountToBeClaimed"),
        borsh.u64("totalAmountClaimed"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new ClaimUserRewardsEvent({
      user: obj.user,
      userMetadata: obj.userMetadata,
      userRewardsVault: obj.userRewardsVault,
      amountToBeClaimed: obj.amountToBeClaimed,
      totalAmountClaimed: obj.totalAmountClaimed,
    })
  }

  static toEncodable(fields: ClaimUserRewardsEventFields) {
    return {
      user: fields.user,
      userMetadata: fields.userMetadata,
      userRewardsVault: fields.userRewardsVault,
      amountToBeClaimed: fields.amountToBeClaimed,
      totalAmountClaimed: fields.totalAmountClaimed,
    }
  }

  toJSON(): ClaimUserRewardsEventJSON {
    return {
      user: this.user.toString(),
      userMetadata: this.userMetadata.toString(),
      userRewardsVault: this.userRewardsVault.toString(),
      amountToBeClaimed: this.amountToBeClaimed.toString(),
      totalAmountClaimed: this.totalAmountClaimed.toString(),
    }
  }

  static fromJSON(obj: ClaimUserRewardsEventJSON): ClaimUserRewardsEvent {
    return new ClaimUserRewardsEvent({
      user: new PublicKey(obj.user),
      userMetadata: new PublicKey(obj.userMetadata),
      userRewardsVault: new PublicKey(obj.userRewardsVault),
      amountToBeClaimed: new BN(obj.amountToBeClaimed),
      totalAmountClaimed: new BN(obj.totalAmountClaimed),
    })
  }

  toEncodable() {
    return ClaimUserRewardsEvent.toEncodable(this)
  }
}
