import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CloseLollyLottoAccounts {
  authority: PublicKey
  lollysLotto: PublicKey
  systemProgram: PublicKey
}

export function closeLollyLotto(
  accounts: CloseLollyLottoAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: true, isWritable: true },
    { pubkey: accounts.lollysLotto, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([22, 35, 127, 44, 232, 130, 126, 47])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
