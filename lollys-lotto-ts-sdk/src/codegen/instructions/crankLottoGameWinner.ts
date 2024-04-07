import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CrankLottoGameWinnerArgs {
  winningNumbers: Array<number>
  winningAmount: BN
}

export interface CrankLottoGameWinnerAccounts {
  authority: PublicKey
  lottoGame: PublicKey
  lottoGameVaultSigner: PublicKey
  lottoGameVault: PublicKey
  tokenProgram: PublicKey
  user: PublicKey
  userMetadata: PublicKey
  userRewardsVault: PublicKey
  lottoTicket: PublicKey
}

export const layout = borsh.struct([
  borsh.array(borsh.u8(), 6, "winningNumbers"),
  borsh.u64("winningAmount"),
])

export function crankLottoGameWinner(
  args: CrankLottoGameWinnerArgs,
  accounts: CrankLottoGameWinnerAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: false, isWritable: false },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: true },
    {
      pubkey: accounts.lottoGameVaultSigner,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.lottoGameVault, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.user, isSigner: false, isWritable: false },
    { pubkey: accounts.userMetadata, isSigner: false, isWritable: true },
    { pubkey: accounts.userRewardsVault, isSigner: false, isWritable: true },
    { pubkey: accounts.lottoTicket, isSigner: false, isWritable: true },
  ]
  const identifier = Buffer.from([138, 129, 143, 247, 7, 218, 25, 202])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      winningNumbers: args.winningNumbers,
      winningAmount: args.winningAmount,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).subarray(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
