import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CreateUserMetadataEventFields {
  user: PublicKey
  userMetadata: PublicKey
  userRewardsVault: PublicKey
  createdTimestamp: BN
}

export interface CreateUserMetadataEventJSON {
  user: string
  userMetadata: string
  userRewardsVault: string
  createdTimestamp: string
}

/** Event emitted when a user creates user metadata. */
export class CreateUserMetadataEvent {
  readonly user: PublicKey
  readonly userMetadata: PublicKey
  readonly userRewardsVault: PublicKey
  readonly createdTimestamp: BN

  constructor(fields: CreateUserMetadataEventFields) {
    this.user = fields.user
    this.userMetadata = fields.userMetadata
    this.userRewardsVault = fields.userRewardsVault
    this.createdTimestamp = fields.createdTimestamp
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("user"),
        borsh.publicKey("userMetadata"),
        borsh.publicKey("userRewardsVault"),
        borsh.i64("createdTimestamp"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CreateUserMetadataEvent({
      user: obj.user,
      userMetadata: obj.userMetadata,
      userRewardsVault: obj.userRewardsVault,
      createdTimestamp: obj.createdTimestamp,
    })
  }

  static toEncodable(fields: CreateUserMetadataEventFields) {
    return {
      user: fields.user,
      userMetadata: fields.userMetadata,
      userRewardsVault: fields.userRewardsVault,
      createdTimestamp: fields.createdTimestamp,
    }
  }

  toJSON(): CreateUserMetadataEventJSON {
    return {
      user: this.user.toString(),
      userMetadata: this.userMetadata.toString(),
      userRewardsVault: this.userRewardsVault.toString(),
      createdTimestamp: this.createdTimestamp.toString(),
    }
  }

  static fromJSON(obj: CreateUserMetadataEventJSON): CreateUserMetadataEvent {
    return new CreateUserMetadataEvent({
      user: new PublicKey(obj.user),
      userMetadata: new PublicKey(obj.userMetadata),
      userRewardsVault: new PublicKey(obj.userRewardsVault),
      createdTimestamp: new BN(obj.createdTimestamp),
    })
  }

  toEncodable() {
    return CreateUserMetadataEvent.toEncodable(this)
  }
}
