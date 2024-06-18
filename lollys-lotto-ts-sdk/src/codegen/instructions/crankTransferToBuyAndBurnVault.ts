import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CrankTransferToBuyAndBurnVaultArgs {
  round: BN
}

export interface CrankTransferToBuyAndBurnVaultAccounts {
  authority: PublicKey
  lottoGame: PublicKey
  lottoGameVaultSigner: PublicKey
  lottoGameVault: PublicKey
  lollyBurnState: PublicKey
  /** LOLLY token account to burn tokens, owned by LollyBurnState PDA */
  lollyBurnStateUsdcVault: PublicKey
  eventEmitter: PublicKey
  tokenProgram: PublicKey
}

export const layout = borsh.struct([borsh.u64("round")])

export function crankTransferToBuyAndBurnVault(
  args: CrankTransferToBuyAndBurnVaultArgs,
  accounts: CrankTransferToBuyAndBurnVaultAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: true },
    {
      pubkey: accounts.lottoGameVaultSigner,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.lottoGameVault, isSigner: false, isWritable: true },
    { pubkey: accounts.lollyBurnState, isSigner: false, isWritable: true },
    {
      pubkey: accounts.lollyBurnStateUsdcVault,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([3, 81, 153, 236, 189, 192, 120, 118])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      round: args.round,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).subarray(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
