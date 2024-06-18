import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface ProcessWinningNumbersAccounts {
  /** We need to make sure the randomness service signed this requests so it can only be invoked by a PDA and not a user. */
  authority: PublicKey
  lottoGame: PublicKey
  randomnessAccountData: PublicKey
  eventEmitter: PublicKey
}

export function processWinningNumbers(
  accounts: ProcessWinningNumbersAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: false, isWritable: false },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: true },
    {
      pubkey: accounts.randomnessAccountData,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
  ]
  const identifier = Buffer.from([124, 211, 139, 177, 171, 186, 119, 217])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
