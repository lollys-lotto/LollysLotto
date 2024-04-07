import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CreateEventEmitterAccounts {
  /** Lamports for rent funded from here. */
  funder: PublicKey
  eventEmitter: PublicKey
  /** Needed to create a new account */
  systemProgram: PublicKey
}

export function createEventEmitter(
  accounts: CreateEventEmitterAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.funder, isSigner: true, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([177, 70, 19, 205, 180, 108, 160, 187])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
