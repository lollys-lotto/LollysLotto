import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CrankLottoGameClosedArgs {
  round: BN
}

export interface CrankLottoGameClosedAccounts {
  authority: PublicKey
  lottoGame: PublicKey
  eventEmitter: PublicKey
}

export const layout = borsh.struct([borsh.u64("round")])

export function crankLottoGameClosed(
  args: CrankLottoGameClosedArgs,
  accounts: CrankLottoGameClosedAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
  ]
  const identifier = Buffer.from([24, 48, 27, 86, 98, 216, 12, 185])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      round: args.round,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
