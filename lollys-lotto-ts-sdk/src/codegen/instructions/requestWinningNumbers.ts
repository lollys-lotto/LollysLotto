import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface RequestWinningNumbersAccounts {
  /** The Solana Randomness Service program. */
  randomnessService: PublicKey
  /**
   * The account that will be created on-chain to hold the randomness request.
   * Used by the off-chain oracle to pickup the request and fulfill it.
   */
  randomnessRequest: PublicKey
  /** The TokenAccount that will store the funds for the randomness request. */
  randomnessEscrow: PublicKey
  /**
   * The randomness service's state account. Responsible for storing the
   * reward escrow and the cost per random byte.
   */
  randomnessState: PublicKey
  /** The token mint to use for paying for randomness requests. */
  randomnessMint: PublicKey
  /** The account that will pay for the randomness request. */
  payer: PublicKey
  authority: PublicKey
  lottoGame: PublicKey
  eventEmitter: PublicKey
  /** The Solana System program. Used to allocate space on-chain for the randomness_request account. */
  systemProgram: PublicKey
  /** The Solana Token program. Used to transfer funds to the randomness escrow. */
  tokenProgram: PublicKey
  /** The Solana Associated Token program. Used to create the TokenAccount for the randomness escrow. */
  associatedTokenProgram: PublicKey
}

export function requestWinningNumbers(
  accounts: RequestWinningNumbersAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.randomnessService, isSigner: false, isWritable: false },
    { pubkey: accounts.randomnessRequest, isSigner: true, isWritable: true },
    { pubkey: accounts.randomnessEscrow, isSigner: false, isWritable: true },
    { pubkey: accounts.randomnessState, isSigner: false, isWritable: false },
    { pubkey: accounts.randomnessMint, isSigner: false, isWritable: false },
    { pubkey: accounts.payer, isSigner: true, isWritable: true },
    { pubkey: accounts.authority, isSigner: false, isWritable: false },
    { pubkey: accounts.lottoGame, isSigner: false, isWritable: false },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    {
      pubkey: accounts.associatedTokenProgram,
      isSigner: false,
      isWritable: false,
    },
  ]
  const identifier = Buffer.from([22, 6, 33, 189, 118, 179, 35, 213])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
