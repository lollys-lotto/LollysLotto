import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface ProcessWinningNumbersArgs {
  result: Uint8Array
}

export interface ProcessWinningNumbersAccounts {
  /** We need to make sure the randomness service signed this requests so it can only be invoked by a PDA and not a user. */
  randomnessState: PublicKey
  request: PublicKey
  authority: PublicKey
  lottoGame: PublicKey
  eventEmitter: PublicKey
}

export const layout = borsh.struct([borsh.vecU8("result")])

export function processWinningNumbers(
  args: ProcessWinningNumbersArgs,
  accounts: ProcessWinningNumbersAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.randomnessState, isSigner: true, isWritable: false },
    { pubkey: accounts.request, isSigner: false, isWritable: false },
    { pubkey: accounts.authority, isSigner: false, isWritable: false },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
  ]
  const identifier = Buffer.from([124, 211, 139, 177, 171, 186, 119, 217])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      result: Buffer.from(
        args.result.buffer,
        args.result.byteOffset,
        args.result.length
      ),
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
