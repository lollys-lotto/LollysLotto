import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CrankTransferToBuyAndBurnVaultEventFields {
  round: BN
  lottoGame: PublicKey
  lollyBurnState: PublicKey
  lollyBurnStateUsdcVault: PublicKey
  lottoGameVault: PublicKey
  buyAndBurnAmount: BN
}

export interface CrankTransferToBuyAndBurnVaultEventJSON {
  round: string
  lottoGame: string
  lollyBurnState: string
  lollyBurnStateUsdcVault: string
  lottoGameVault: string
  buyAndBurnAmount: string
}

export class CrankTransferToBuyAndBurnVaultEvent {
  readonly round: BN
  readonly lottoGame: PublicKey
  readonly lollyBurnState: PublicKey
  readonly lollyBurnStateUsdcVault: PublicKey
  readonly lottoGameVault: PublicKey
  readonly buyAndBurnAmount: BN

  constructor(fields: CrankTransferToBuyAndBurnVaultEventFields) {
    this.round = fields.round
    this.lottoGame = fields.lottoGame
    this.lollyBurnState = fields.lollyBurnState
    this.lollyBurnStateUsdcVault = fields.lollyBurnStateUsdcVault
    this.lottoGameVault = fields.lottoGameVault
    this.buyAndBurnAmount = fields.buyAndBurnAmount
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u64("round"),
        borsh.publicKey("lottoGame"),
        borsh.publicKey("lollyBurnState"),
        borsh.publicKey("lollyBurnStateUsdcVault"),
        borsh.publicKey("lottoGameVault"),
        borsh.u64("buyAndBurnAmount"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CrankTransferToBuyAndBurnVaultEvent({
      round: obj.round,
      lottoGame: obj.lottoGame,
      lollyBurnState: obj.lollyBurnState,
      lollyBurnStateUsdcVault: obj.lollyBurnStateUsdcVault,
      lottoGameVault: obj.lottoGameVault,
      buyAndBurnAmount: obj.buyAndBurnAmount,
    })
  }

  static toEncodable(fields: CrankTransferToBuyAndBurnVaultEventFields) {
    return {
      round: fields.round,
      lottoGame: fields.lottoGame,
      lollyBurnState: fields.lollyBurnState,
      lollyBurnStateUsdcVault: fields.lollyBurnStateUsdcVault,
      lottoGameVault: fields.lottoGameVault,
      buyAndBurnAmount: fields.buyAndBurnAmount,
    }
  }

  toJSON(): CrankTransferToBuyAndBurnVaultEventJSON {
    return {
      round: this.round.toString(),
      lottoGame: this.lottoGame.toString(),
      lollyBurnState: this.lollyBurnState.toString(),
      lollyBurnStateUsdcVault: this.lollyBurnStateUsdcVault.toString(),
      lottoGameVault: this.lottoGameVault.toString(),
      buyAndBurnAmount: this.buyAndBurnAmount.toString(),
    }
  }

  static fromJSON(
    obj: CrankTransferToBuyAndBurnVaultEventJSON
  ): CrankTransferToBuyAndBurnVaultEvent {
    return new CrankTransferToBuyAndBurnVaultEvent({
      round: new BN(obj.round),
      lottoGame: new PublicKey(obj.lottoGame),
      lollyBurnState: new PublicKey(obj.lollyBurnState),
      lollyBurnStateUsdcVault: new PublicKey(obj.lollyBurnStateUsdcVault),
      lottoGameVault: new PublicKey(obj.lottoGameVault),
      buyAndBurnAmount: new BN(obj.buyAndBurnAmount),
    })
  }

  toEncodable() {
    return CrankTransferToBuyAndBurnVaultEvent.toEncodable(this)
  }
}
