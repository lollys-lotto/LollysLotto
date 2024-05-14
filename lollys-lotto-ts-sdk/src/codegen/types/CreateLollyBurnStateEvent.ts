import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CreateLollyBurnStateEventFields {
  authority: PublicKey
  lollyBurnState: PublicKey
  lollyMint: PublicKey
  lollyBurnStateLollyVault: PublicKey
  usdcMint: PublicKey
  lollyBurnStateUsdcVault: PublicKey
}

export interface CreateLollyBurnStateEventJSON {
  authority: string
  lollyBurnState: string
  lollyMint: string
  lollyBurnStateLollyVault: string
  usdcMint: string
  lollyBurnStateUsdcVault: string
}

/** Event emitted when a user creates a lolly burn state. */
export class CreateLollyBurnStateEvent {
  readonly authority: PublicKey
  readonly lollyBurnState: PublicKey
  readonly lollyMint: PublicKey
  readonly lollyBurnStateLollyVault: PublicKey
  readonly usdcMint: PublicKey
  readonly lollyBurnStateUsdcVault: PublicKey

  constructor(fields: CreateLollyBurnStateEventFields) {
    this.authority = fields.authority
    this.lollyBurnState = fields.lollyBurnState
    this.lollyMint = fields.lollyMint
    this.lollyBurnStateLollyVault = fields.lollyBurnStateLollyVault
    this.usdcMint = fields.usdcMint
    this.lollyBurnStateUsdcVault = fields.lollyBurnStateUsdcVault
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("authority"),
        borsh.publicKey("lollyBurnState"),
        borsh.publicKey("lollyMint"),
        borsh.publicKey("lollyBurnStateLollyVault"),
        borsh.publicKey("usdcMint"),
        borsh.publicKey("lollyBurnStateUsdcVault"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CreateLollyBurnStateEvent({
      authority: obj.authority,
      lollyBurnState: obj.lollyBurnState,
      lollyMint: obj.lollyMint,
      lollyBurnStateLollyVault: obj.lollyBurnStateLollyVault,
      usdcMint: obj.usdcMint,
      lollyBurnStateUsdcVault: obj.lollyBurnStateUsdcVault,
    })
  }

  static toEncodable(fields: CreateLollyBurnStateEventFields) {
    return {
      authority: fields.authority,
      lollyBurnState: fields.lollyBurnState,
      lollyMint: fields.lollyMint,
      lollyBurnStateLollyVault: fields.lollyBurnStateLollyVault,
      usdcMint: fields.usdcMint,
      lollyBurnStateUsdcVault: fields.lollyBurnStateUsdcVault,
    }
  }

  toJSON(): CreateLollyBurnStateEventJSON {
    return {
      authority: this.authority.toString(),
      lollyBurnState: this.lollyBurnState.toString(),
      lollyMint: this.lollyMint.toString(),
      lollyBurnStateLollyVault: this.lollyBurnStateLollyVault.toString(),
      usdcMint: this.usdcMint.toString(),
      lollyBurnStateUsdcVault: this.lollyBurnStateUsdcVault.toString(),
    }
  }

  static fromJSON(
    obj: CreateLollyBurnStateEventJSON
  ): CreateLollyBurnStateEvent {
    return new CreateLollyBurnStateEvent({
      authority: new PublicKey(obj.authority),
      lollyBurnState: new PublicKey(obj.lollyBurnState),
      lollyMint: new PublicKey(obj.lollyMint),
      lollyBurnStateLollyVault: new PublicKey(obj.lollyBurnStateLollyVault),
      usdcMint: new PublicKey(obj.usdcMint),
      lollyBurnStateUsdcVault: new PublicKey(obj.lollyBurnStateUsdcVault),
    })
  }

  toEncodable() {
    return CreateLollyBurnStateEvent.toEncodable(this)
  }
}
