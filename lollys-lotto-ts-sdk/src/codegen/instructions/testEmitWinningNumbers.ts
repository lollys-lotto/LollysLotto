import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface TestEmitWinningNumbersArgs {
  result: Array<number>
}

export interface TestEmitWinningNumbersAccounts {
  authority: PublicKey
  lottoGame: PublicKey
  eventEmitter: PublicKey
}

export const layout = borsh.struct([borsh.array(borsh.u8(), 32, "result")])

export function testEmitWinningNumbers(
  args: TestEmitWinningNumbersArgs,
  accounts: TestEmitWinningNumbersAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: false, isWritable: false },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
  ]
  const identifier = Buffer.from([108, 205, 75, 141, 80, 112, 48, 145])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      result: args.result,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).subarray(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
