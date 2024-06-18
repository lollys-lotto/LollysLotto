import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CrankLottoGameWinnersArgs {
  round: BN
  winningNumbers: types.LottoTicketNumbersFields
  winningNumbersIndex: Array<BN>
}

export interface CrankLottoGameWinnersAccounts {
  authority: PublicKey
  lottoGame: PublicKey
  lottoGameVaultSigner: PublicKey
  lottoGameVault: PublicKey
  user: PublicKey
  userMetadata: PublicKey
  lottoTicket: PublicKey
  eventEmitter: PublicKey
  tokenProgram: PublicKey
}

export const layout = borsh.struct([
  borsh.u64("round"),
  types.LottoTicketNumbers.layout("winningNumbers"),
  borsh.array(borsh.i64(), 4, "winningNumbersIndex"),
])

export function crankLottoGameWinners(
  args: CrankLottoGameWinnersArgs,
  accounts: CrankLottoGameWinnersAccounts,
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
    { pubkey: accounts.lottoTicket, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([66, 116, 168, 13, 36, 25, 223, 44])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      round: args.round,
      winningNumbers: types.LottoTicketNumbers.toEncodable(args.winningNumbers),
      winningNumbersIndex: args.winningNumbersIndex,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).subarray(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
