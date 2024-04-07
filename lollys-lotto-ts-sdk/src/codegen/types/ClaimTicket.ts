import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface ClaimTicketFields {
  claimedAmount: BN
  createdAt: BN
}

export interface ClaimTicketJSON {
  claimedAmount: string
  createdAt: string
}

export class ClaimTicket {
  readonly claimedAmount: BN
  readonly createdAt: BN

  constructor(fields: ClaimTicketFields) {
    this.claimedAmount = fields.claimedAmount
    this.createdAt = fields.createdAt
  }

  static layout(property?: string) {
    return borsh.struct(
      [borsh.u64("claimedAmount"), borsh.i64("createdAt")],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new ClaimTicket({
      claimedAmount: obj.claimedAmount,
      createdAt: obj.createdAt,
    })
  }

  static toEncodable(fields: ClaimTicketFields) {
    return {
      claimedAmount: fields.claimedAmount,
      createdAt: fields.createdAt,
    }
  }

  toJSON(): ClaimTicketJSON {
    return {
      claimedAmount: this.claimedAmount.toString(),
      createdAt: this.createdAt.toString(),
    }
  }

  static fromJSON(obj: ClaimTicketJSON): ClaimTicket {
    return new ClaimTicket({
      claimedAmount: new BN(obj.claimedAmount),
      createdAt: new BN(obj.createdAt),
    })
  }

  toEncodable() {
    return ClaimTicket.toEncodable(this)
  }
}
