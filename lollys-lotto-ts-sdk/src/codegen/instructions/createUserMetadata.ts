import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CreateUserMetadataAccounts {
  user: PublicKey
  userMetadata: PublicKey
  /** Mint address of the USDC token */
  usdcMint: PublicKey
  userRewardsVault: PublicKey
  eventEmitter: PublicKey
  tokenProgram: PublicKey
  associatedTokenProgram: PublicKey
  systemProgram: PublicKey
}

export function createUserMetadata(
  accounts: CreateUserMetadataAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.user, isSigner: true, isWritable: true },
    { pubkey: accounts.userMetadata, isSigner: false, isWritable: true },
    { pubkey: accounts.usdcMint, isSigner: false, isWritable: false },
    { pubkey: accounts.userRewardsVault, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    {
      pubkey: accounts.associatedTokenProgram,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([56, 247, 226, 15, 184, 166, 38, 102])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
