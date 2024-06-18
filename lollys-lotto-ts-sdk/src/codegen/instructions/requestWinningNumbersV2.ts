import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface RequestWinningNumbersV2Accounts {
  authority: PublicKey
  lottoGame: PublicKey
  randomnessAccountData: PublicKey
  eventEmitter: PublicKey
  /** The Solana System program. Used to allocate space on-chain for the randomness_request account. */
  systemProgram: PublicKey
}

export function requestWinningNumbersV2(
  accounts: RequestWinningNumbersV2Accounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: false, isWritable: false },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: false },
    {
      pubkey: accounts.randomnessAccountData,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([94, 19, 50, 41, 216, 5, 2, 148])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
