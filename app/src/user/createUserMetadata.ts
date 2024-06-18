import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import { 
  createUserMetadata, 
  CreateUserMetadataAccounts,
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

export async function createUserMetadataIfNotExist(
    lollysLottoProgram: anchor.Program,
    user: Keypair,
    eventEmitterPda: PublicKey,
    connection: Connection,
  ): Promise<{ userMetadataPda: PublicKey, userRewardsVault: PublicKey}>  {
      const userMetadataPda = getUserMetadataPda(user.publicKey, lollysLottoProgram.programId);
      const userRewardsVault = await getAssociatedTokenAddress(USDC_MINT_DEVNET, userMetadataPda, true);
      console.log(`User Rewards Vault ${userRewardsVault.toBase58()}`);
  
      try {
          const userMetadataAccount = await lollysLottoProgram.account.userMetadata.fetch(userMetadataPda);
          console.log(`User Metadata Bump ${JSON.stringify(userMetadataAccount.bump)}`);
      } catch {
          console.log(`User Metadata Account does not exist, creating...`);
          
          const createUserMetadataAccounts: CreateUserMetadataAccounts = {
              user: user.publicKey,
              userMetadata: userMetadataPda,
              usdcMint: USDC_MINT_DEVNET,
              userRewardsVault,
              eventEmitter: eventEmitterPda,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
              systemProgram: SystemProgram.programId,
          };
  
          const createUserMetadataIx = createUserMetadata(createUserMetadataAccounts, lollysLottoProgram.programId);
          const sig = await sendVersionedTx(connection, [createUserMetadataIx], user.publicKey, [user]);
          console.log(`[TX] createUserMetadata: ${sig}`);
      }
      
      return {
        userMetadataPda,
        userRewardsVault,
      };
  }
  