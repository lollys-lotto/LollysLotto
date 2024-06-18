import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CloseLottoTicketArgs {
  round: BN
  numbers: types.LottoTicketNumbersFields
}

export interface CloseLottoTicketAccounts {
  authority: PublicKey
  user: PublicKey
  userMetadata: PublicKey
  lottoGame: PublicKey
  lottoTicket: PublicKey
  eventEmitter: PublicKey
  systemProgram: PublicKey
}

export const layout = borsh.struct([
  borsh.u64("round"),
  types.LottoTicketNumbers.layout("numbers"),
])

export function closeLottoTicket(
  args: CloseLottoTicketArgs,
  accounts: CloseLottoTicketAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: false, isWritable: false },
    { pubkey: accounts.user, isSigner: true, isWritable: true },
    { pubkey: accounts.userMetadata, isSigner: false, isWritable: true },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: false },
    { pubkey: accounts.lottoTicket, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([49, 163, 44, 78, 48, 118, 95, 1])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      round: args.round,
      numbers: types.LottoTicketNumbers.toEncodable(args.numbers),
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).subarray(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
