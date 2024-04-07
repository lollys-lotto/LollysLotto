import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CloseLottoGameAccounts {
  authority: PublicKey
  lottoGame: PublicKey
  systemProgram: PublicKey
}

export function closeLottoGame(
  accounts: CloseLottoGameAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: true, isWritable: true },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([73, 152, 130, 19, 160, 60, 59, 224])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
