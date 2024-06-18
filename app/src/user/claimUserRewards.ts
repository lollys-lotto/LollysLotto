import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import { 
    claimUserRewards, ClaimUserRewardsAccounts, ClaimUserRewardsArgs,
} from "../../../lollys-lotto-ts-sdk/src/codegen/instructions";
import { 
    ASSOCIATED_TOKEN_PROGRAM_ID, 
    TOKEN_PROGRAM_ID, 
    getAssociatedTokenAddress 
} from "@solana/spl-token";
import { 
  getUserMetadataPda, 
  sendVersionedTx 
} from "../utils";
import { USDC_MINT_DEVNET } from "../constants";

export async function claimUserRewardsTx(
    lollysLottoProgram: anchor.Program,
    amountToBeClaimed: anchor.BN,
    user: Keypair,
    eventEmitterPda: PublicKey,
    connection: Connection,
  ): Promise<string>  {
      const userMetadataPda = getUserMetadataPda(user.publicKey, lollysLottoProgram.programId);
      const userUsdcTokenAccount = await getAssociatedTokenAddress(USDC_MINT_DEVNET, user.publicKey, false);
      const userRewardsVault = await getAssociatedTokenAddress(USDC_MINT_DEVNET, userMetadataPda, true);
      console.log("User Metadata PDA: ", userMetadataPda.toBase58());
      console.log("User USDC Token Account: ", userUsdcTokenAccount.toBase58());
      console.log(`User Rewards Vault ${userRewardsVault.toBase58()}`);
      let sig: string;
      try {
        console.log(`Claiming user rewards for ${user.publicKey.toBase58()}...`);
        
        const claimUserRewardsArgs : ClaimUserRewardsArgs = {
          amountToBeClaimed,
        }
        const claimUserRewardsAccounts: ClaimUserRewardsAccounts = {
            user: user.publicKey,
            userUsdcTokenAccount,
            userMetadata: userMetadataPda,
            usdcMint: USDC_MINT_DEVNET,
            userRewardsVault,
            eventEmitter: eventEmitterPda,
            tokenProgram: TOKEN_PROGRAM_ID,
        };

        const claimUserRewardsIx = claimUserRewards(claimUserRewardsArgs, claimUserRewardsAccounts, lollysLottoProgram.programId);
        sig = await sendVersionedTx(connection, [claimUserRewardsIx], user.publicKey, [user]);
        console.log(`[TX] createUserMetadata: ${sig}`);
      } catch {
        console.error(`Error claiming user rewards for ${user.publicKey.toBase58()}`);
      }
      
      return sig;
  }
  