import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CrankTransferWinningAmountToUserRewardsVaultArgs {
  round: BN
  winningNumbers: types.LottoTicketNumbersFields
  numberOfTicketsWithDuplicateNumbers: number
}

export interface CrankTransferWinningAmountToUserRewardsVaultAccounts {
  authority: PublicKey
  lottoGame: PublicKey
  lottoGameVaultSigner: PublicKey
  lottoGameVault: PublicKey
  user: PublicKey
  userMetadata: PublicKey
  userRewardsVault: PublicKey
  lottoTicket: PublicKey
  eventEmitter: PublicKey
  tokenProgram: PublicKey
}

export const layout = borsh.struct([
  borsh.u64("round"),
  types.LottoTicketNumbers.layout("winningNumbers"),
  borsh.u32("numberOfTicketsWithDuplicateNumbers"),
])

export function crankTransferWinningAmountToUserRewardsVault(
  args: CrankTransferWinningAmountToUserRewardsVaultArgs,
  accounts: CrankTransferWinningAmountToUserRewardsVaultAccounts,
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
    { pubkey: accounts.user, isSigner: false, isWritable: false },
    { pubkey: accounts.userMetadata, isSigner: false, isWritable: true },
    { pubkey: accounts.userRewardsVault, isSigner: false, isWritable: true },
    { pubkey: accounts.lottoTicket, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([101, 178, 55, 235, 11, 59, 135, 113])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      round: args.round,
      winningNumbers: types.LottoTicketNumbers.toEncodable(args.winningNumbers),
      numberOfTicketsWithDuplicateNumbers:
        args.numberOfTicketsWithDuplicateNumbers,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).subarray(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
