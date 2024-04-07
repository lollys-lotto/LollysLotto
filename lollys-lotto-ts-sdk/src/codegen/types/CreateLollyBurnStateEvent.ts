import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CreateLollyBurnStateEventFields {
  user: PublicKey
  lollyBurnState: PublicKey
}

export interface CreateLollyBurnStateEventJSON {
  user: string
  lollyBurnState: string
}

/** Event emitted when a user creates a lolly burn state. */
export class CreateLollyBurnStateEvent {
  readonly user: PublicKey
  readonly lollyBurnState: PublicKey

  constructor(fields: CreateLollyBurnStateEventFields) {
    this.user = fields.user
    this.lollyBurnState = fields.lollyBurnState
  }

  static layout(property?: string) {
    return borsh.struct(
      [borsh.publicKey("user"), borsh.publicKey("lollyBurnState")],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CreateLollyBurnStateEvent({
      user: obj.user,
      lollyBurnState: obj.lollyBurnState,
    })
  }

  static toEncodable(fields: CreateLollyBurnStateEventFields) {
    return {
      user: fields.user,
      lollyBurnState: fields.lollyBurnState,
    }
  }

  toJSON(): CreateLollyBurnStateEventJSON {
    return {
      user: this.user.toString(),
      lollyBurnState: this.lollyBurnState.toString(),
    }
  }

  static fromJSON(
    obj: CreateLollyBurnStateEventJSON
  ): CreateLollyBurnStateEvent {
    return new CreateLollyBurnStateEvent({
      user: new PublicKey(obj.user),
      lollyBurnState: new PublicKey(obj.lollyBurnState),
    })
  }

  toEncodable() {
    return CreateLollyBurnStateEvent.toEncodable(this)
  }
}
