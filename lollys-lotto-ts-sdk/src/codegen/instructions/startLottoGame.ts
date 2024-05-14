import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface StartLottoGameArgs {
  round: BN
  ticketPrice: BN
  gameDuration: BN
  roundName: string
}

export interface StartLottoGameAccounts {
  authority: PublicKey
  lollysLotto: PublicKey
  lottoGame: PublicKey
  lottoGameVaultSigner: PublicKey
  /**
   * This instruction initializes the token account
   * required for storing this LottoGame's ticket amount collections in USDC.
   */
  lottoGameVault: PublicKey
  /** Needed for account initialization */
  lottoGameMint: PublicKey
  eventEmitter: PublicKey
  tokenProgram: PublicKey
  associatedTokenProgram: PublicKey
  systemProgram: PublicKey
}

export const layout = borsh.struct([
  borsh.u64("round"),
  borsh.u64("ticketPrice"),
  borsh.u64("gameDuration"),
  borsh.str("roundName"),
])

export function startLottoGame(
  args: StartLottoGameArgs,
  accounts: StartLottoGameAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: true, isWritable: true },
    { pubkey: accounts.lollysLotto, isSigner: false, isWritable: true },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: true },
    {
      pubkey: accounts.lottoGameVaultSigner,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.lottoGameVault, isSigner: false, isWritable: true },
    { pubkey: accounts.lottoGameMint, isSigner: false, isWritable: false },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    {
      pubkey: accounts.associatedTokenProgram,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([170, 200, 70, 202, 157, 229, 45, 144])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      round: args.round,
      ticketPrice: args.ticketPrice,
      gameDuration: args.gameDuration,
      roundName: args.roundName,
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
