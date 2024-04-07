import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface BurnLollyAccounts {
  /** This is the token mint that we want to burn */
  lollyMint: PublicKey
  /** The authority of the LollyBurnState instance */
  authority: PublicKey
  lollyBurnState: PublicKey
  /** LOLLY token account to burn tokens, owned by LollyBurnState PDA */
  lollyBurnStateLollyVault: PublicKey
  tokenProgram: PublicKey
}

export function burnLolly(
  accounts: BurnLollyAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.lollyMint, isSigner: false, isWritable: false },
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    { pubkey: accounts.lollyBurnState, isSigner: false, isWritable: true },
    {
      pubkey: accounts.lollyBurnStateLollyVault,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([252, 133, 232, 22, 170, 186, 25, 139])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
