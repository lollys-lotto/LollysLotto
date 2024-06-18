import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface SwapUsdcLollyArgs {
  data: Uint8Array
}

export interface SwapUsdcLollyAccounts {
  authority: PublicKey
  lollyBurnState: PublicKey
  /**
   * token_in_mint to be swapped using jupiter
   * Mint address of the USDC token
   * associated_token_account of token_in_mint
   * USDC token account which is used to swap USDC to LOLLY using jupiter owned by LollyBurnState PDA
   */
  lollyBurnStateUsdcVault: PublicKey
  /**
   * token_out_mint to be swapped using jupiter
   * Mint address of the LOLLY token
   * associated_token_account of token_out_mint
   * LOLLY token account to store LOLLY swapped from USDC of lolly_burn_state_usdc_vault using jupiter owned by LollyBurnState PDA
   */
  lollyBurnStateLollyVault: PublicKey
  eventEmitter: PublicKey
  jupiterProgram: PublicKey
  tokenProgram: PublicKey
  systemProgram: PublicKey
}

export const layout = borsh.struct([borsh.vecU8("data")])

export function swapUsdcLolly(
  args: SwapUsdcLollyArgs,
  accounts: SwapUsdcLollyAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    { pubkey: accounts.lollyBurnState, isSigner: false, isWritable: true },
    {
      pubkey: accounts.lollyBurnStateUsdcVault,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: accounts.lollyBurnStateLollyVault,
      isSigner: false,
      isWritable: true,
    },
    { pubkey: accounts.eventEmitter, isSigner: false, isWritable: true },
    { pubkey: accounts.jupiterProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.tokenProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([186, 169, 100, 138, 156, 139, 160, 75])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      data: Buffer.from(
        args.data.buffer,
        args.data.byteOffset,
        args.data.length
      ),
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).subarray(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
