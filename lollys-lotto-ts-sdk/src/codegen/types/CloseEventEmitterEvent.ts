import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CloseEventEmitterEventFields {
  eventEmitter: PublicKey
}

export interface CloseEventEmitterEventJSON {
  eventEmitter: string
}

/** Event emitted when a user closes EventEmitter state. */
export class CloseEventEmitterEvent {
  readonly eventEmitter: PublicKey

  constructor(fields: CloseEventEmitterEventFields) {
    this.eventEmitter = fields.eventEmitter
  }

  static layout(property?: string) {
    return borsh.struct([borsh.publicKey("eventEmitter")], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CloseEventEmitterEvent({
      eventEmitter: obj.eventEmitter,
    })
  }

  static toEncodable(fields: CloseEventEmitterEventFields) {
    return {
      eventEmitter: fields.eventEmitter,
    }
  }

  toJSON(): CloseEventEmitterEventJSON {
    return {
      eventEmitter: this.eventEmitter.toString(),
    }
  }

  static fromJSON(obj: CloseEventEmitterEventJSON): CloseEventEmitterEvent {
    return new CloseEventEmitterEvent({
      eventEmitter: new PublicKey(obj.eventEmitter),
    })
  }

  toEncodable() {
    return CloseEventEmitterEvent.toEncodable(this)
  }
}
