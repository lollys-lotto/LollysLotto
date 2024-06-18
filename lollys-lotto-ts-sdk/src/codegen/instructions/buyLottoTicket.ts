import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface BuyLottoTicketArgs {
  round: BN
  lottoTicketNumbers: types.LottoTicketNumbersFields
}

export interface BuyLottoTicketAccounts {
  authority: PublicKey
  user: PublicKey
  userMetadata: PublicKey
  userUsdcTokenAccount: PublicKey
  lottoGameMint: PublicKey
  lottoGame: PublicKey
  lottoGameVault: PublicKey
  lottoTicket: PublicKey
  eventEmitter: PublicKey
  tokenProgram: PublicKey
  associatedTokenProgram: PublicKey
  systemProgram: PublicKey
}

export const layout = borsh.struct([
  borsh.u64("round"),
  types.LottoTicketNumbers.layout("lottoTicketNumbers"),
])

export function buyLottoTicket(
  args: BuyLottoTicketArgs,
  accounts: BuyLottoTicketAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: false, isWritable: false },
    { pubkey: accounts.user, isSigner: true, isWritable: true },
    { pubkey: accounts.userMetadata, isSigner: false, isWritable: true },
    {
      pubkey: accounts.userUsdcTokenAccount,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.lottoGameMint, isSigner: false, isWritable: false },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: true },
    { pubkey: accounts.lottoGameVault, isSigner: false, isWritable: true },
    { pubkey: accounts.lottoTicket, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    {
      pubkey: accounts.associatedTokenProgram,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([183, 179, 100, 99, 208, 96, 97, 49])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      round: args.round,
      lottoTicketNumbers: types.LottoTicketNumbers.toEncodable(
        args.lottoTicketNumbers
      ),
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).subarray(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
