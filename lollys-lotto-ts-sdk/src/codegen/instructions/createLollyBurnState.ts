import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface CreateLollyBurnStateAccounts {
  payer: PublicKey
  authority: PublicKey
  /** LollyBurnState instance to be created */
  lollyBurnState: PublicKey
  /** Mint address of the LOLLY token */
  lollyMint: PublicKey
  /** LOLLY token account to store LOLLY swapped from USDC of lolly_burn_state_usdc_vault using jupiter owned by LollyBurnState PDA */
  lollyBurnStateLollyVault: PublicKey
  /** Mint address of the USDC token */
  usdcMint: PublicKey
  /** USDC token account to store USDC sent from LottoGame USDC vault owned by LollyBurnState PDA */
  lollyBurnStateUsdcVault: PublicKey
  eventEmitter: PublicKey
  tokenProgram: PublicKey
  associatedTokenProgram: PublicKey
  systemProgram: PublicKey
}

export function createLollyBurnState(
  accounts: CreateLollyBurnStateAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.payer, isSigner: true, isWritable: true },
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    { pubkey: accounts.lollyBurnState, isSigner: false, isWritable: true },
    { pubkey: accounts.lollyMint, isSigner: false, isWritable: false },
    {
      pubkey: accounts.lollyBurnStateLollyVault,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.usdcMint, isSigner: false, isWritable: false },
    {
      pubkey: accounts.lollyBurnStateUsdcVault,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    {
      pubkey: accounts.associatedTokenProgram,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([3, 148, 18, 19, 17, 23, 79, 232])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
