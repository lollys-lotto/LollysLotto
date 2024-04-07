import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CreateUserMetadataEventFields {
  user: PublicKey
  userMetadata: PublicKey
  createdTimestamp: BN
}

export interface CreateUserMetadataEventJSON {
  user: string
  userMetadata: string
  createdTimestamp: string
}

/** Event emitted when a user creates user metadata. */
export class CreateUserMetadataEvent {
  readonly user: PublicKey
  readonly userMetadata: PublicKey
  readonly createdTimestamp: BN

  constructor(fields: CreateUserMetadataEventFields) {
    this.user = fields.user
    this.userMetadata = fields.userMetadata
    this.createdTimestamp = fields.createdTimestamp
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("user"),
        borsh.publicKey("userMetadata"),
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
      createdTimestamp: obj.createdTimestamp,
    })
  }

  static toEncodable(fields: CreateUserMetadataEventFields) {
    return {
      user: fields.user,
      userMetadata: fields.userMetadata,
      createdTimestamp: fields.createdTimestamp,
    }
  }

  toJSON(): CreateUserMetadataEventJSON {
    return {
      user: this.user.toString(),
      userMetadata: this.userMetadata.toString(),
      createdTimestamp: this.createdTimestamp.toString(),
    }
  }

  static fromJSON(obj: CreateUserMetadataEventJSON): CreateUserMetadataEvent {
    return new CreateUserMetadataEvent({
      user: new PublicKey(obj.user),
      userMetadata: new PublicKey(obj.userMetadata),
      createdTimestamp: new BN(obj.createdTimestamp),
    })
  }

  toEncodable() {
    return CreateUserMetadataEvent.toEncodable(this)
  }
}
