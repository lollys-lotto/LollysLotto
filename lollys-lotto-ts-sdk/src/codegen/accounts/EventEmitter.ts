import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface EventEmitterFields {
  /** One-up, for tracking gaps in recorded program history */
  eventId: BN
}

export interface EventEmitterJSON {
  /** One-up, for tracking gaps in recorded program history */
  eventId: string
}

/** Tracker for event emission. */
export class EventEmitter {
  /** One-up, for tracking gaps in recorded program history */
  readonly eventId: BN

  static readonly discriminator = Buffer.from([
    215, 12, 224, 60, 205, 124, 188, 73,
  ])

  static readonly layout = borsh.struct([borsh.i64("eventId")])

  constructor(fields: EventEmitterFields) {
    this.eventId = fields.eventId
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<EventEmitter | null> {
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
  ): Promise<Array<EventEmitter | null>> {
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

  static decode(data: Buffer): EventEmitter {
    if (!data.subarray(0, 8).equals(EventEmitter.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = EventEmitter.layout.decode(data.subarray(8))

    return new EventEmitter({
      eventId: dec.eventId,
    })
  }

  toJSON(): EventEmitterJSON {
    return {
      eventId: this.eventId.toString(),
    }
  }

  static fromJSON(obj: EventEmitterJSON): EventEmitter {
    return new EventEmitter({
      eventId: new BN(obj.eventId),
    })
  }
}
