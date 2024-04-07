import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CreateLollysLottoAccounts {
  authority: PublicKey
  lollysLotto: PublicKey
  eventEmitter: PublicKey
  systemProgram: PublicKey
}

export function createLollysLotto(
  accounts: CreateLollysLottoAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: true, isWritable: true },
    { pubkey: accounts.lollysLotto, isSigner: false, isWritable: true },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([72, 172, 241, 137, 98, 187, 224, 41])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
