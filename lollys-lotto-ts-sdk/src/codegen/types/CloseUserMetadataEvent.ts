import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CloseUserMetadataEventFields {
  userMetadata: PublicKey
}

export interface CloseUserMetadataEventJSON {
  userMetadata: string
}

export class CloseUserMetadataEvent {
  readonly userMetadata: PublicKey

  constructor(fields: CloseUserMetadataEventFields) {
    this.userMetadata = fields.userMetadata
  }

  static layout(property?: string) {
    return borsh.struct([borsh.publicKey("userMetadata")], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CloseUserMetadataEvent({
      userMetadata: obj.userMetadata,
    })
  }

  static toEncodable(fields: CloseUserMetadataEventFields) {
    return {
      userMetadata: fields.userMetadata,
    }
  }

  toJSON(): CloseUserMetadataEventJSON {
    return {
      userMetadata: this.userMetadata.toString(),
    }
  }

  static fromJSON(obj: CloseUserMetadataEventJSON): CloseUserMetadataEvent {
    return new CloseUserMetadataEvent({
      userMetadata: new PublicKey(obj.userMetadata),
    })
  }

  toEncodable() {
    return CloseUserMetadataEvent.toEncodable(this)
  }
}
