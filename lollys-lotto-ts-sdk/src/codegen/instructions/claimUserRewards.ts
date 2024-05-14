import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface ClaimUserRewardsArgs {
  amountToBeClaimed: BN
}

export interface ClaimUserRewardsAccounts {
  user: PublicKey
  userUsdcTokenAccount: PublicKey
  userMetadata: PublicKey
  /** Mint address of the USDC token */
  usdcMint: PublicKey
  userRewardsVault: PublicKey
  eventEmitter: PublicKey
  tokenProgram: PublicKey
}

export const layout = borsh.struct([borsh.u64("amountToBeClaimed")])

export function claimUserRewards(
  args: ClaimUserRewardsArgs,
  accounts: ClaimUserRewardsAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.user, isSigner: true, isWritable: false },
    {
      pubkey: accounts.userUsdcTokenAccount,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.userMetadata, isSigner: false, isWritable: true },
    { pubkey: accounts.usdcMint, isSigner: false, isWritable: false },
    { pubkey: accounts.userRewardsVault, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([219, 205, 212, 97, 101, 221, 21, 245])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      amountToBeClaimed: args.amountToBeClaimed,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
