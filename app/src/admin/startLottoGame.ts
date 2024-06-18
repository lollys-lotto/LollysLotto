import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  Keypair,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { 
  startLottoGame, 
  StartLottoGameArgs, 
  StartLottoGameAccounts
} from "../../../lollys-lotto-ts-sdk/src/codegen/instructions";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { 
  getLottoGamePda, 
  getLottoGameVaultSignerPda, 
  getLottoGameVaultPda, 
} from "../utils";
import { USDC_MINT_DEVNET } from "../constants";
import { LottoGame } from "../../../lollys-lotto-ts-sdk/src/codegen/accounts/LottoGame";


export async function startLottoGameIxIfNotExist(
  lollysLottoProgram: anchor.Program,
  authority: Keypair,
  round: anchor.BN,
  ticketPrice: anchor.BN,
  gameDuration: anchor.BN,
  roundName: string,
  randomnessAccount: PublicKey,
  lollysLottoPda: PublicKey,
  eventEmitterPda: PublicKey,
  connection: Connection,
): Promise<{ lottoGamePda: PublicKey, lottoGameVaultPda: PublicKey, lottoGameVaultSigner: PublicKey, startLottoGameIx: TransactionInstruction }>  {

  const lottoGamePda = getLottoGamePda(authority.publicKey, round, lollysLottoProgram.programId);
  const lottoGameVaultSigner = getLottoGameVaultSignerPda(lottoGamePda, lollysLottoProgram.programId);
  const lottoGameVaultPda = await getLottoGameVaultPda(lottoGameVaultSigner);

  let startLottoGameIx: TransactionInstruction | null = null;

  try {
    console.log("Lotto Game Account found, fetching...");
    
    // const lottoGameAccount = await lollysLottoProgram.account.lottoGame.fetchAndContext(lottoGamePda);
    const lottoGameAccount = await LottoGame.fetch(connection, lottoGamePda);
    console.log(`Lotto Game Account Bump: ${JSON.stringify(lottoGameAccount.bump)}`);
    console.log(`Lotto Game Account jackpotWinningNumbers: ${JSON.stringify(lottoGameAccount.jackpotWinningNumbers)}`);
    console.log(`Lotto Game Account tier1WinningNumbers: ${JSON.stringify(lottoGameAccount.tier1WinningNumbers)}`);
    // console.log(`Lotto Game Account context: ${lottoGameAccount.context}`);
    
    // console.log(`Lotto Game Account tier2WinningNumbers: ${JSON.stringify(lottoGameAccount.data.tier2WinningNumbers)}`);
    // console.log(`Lotto Game Account tier3WinningNumbers: ${JSON.stringify(lottoGameAccount.data.tier3WinningNumbers)}`);
    
  } catch {
    console.log("Lotto Game Account not found, creating...");
    const startLottoGameArgs: StartLottoGameArgs = {
      round: round,
      ticketPrice: ticketPrice,
      gameDuration: gameDuration,
      randomnessAccount: randomnessAccount,
      roundName: roundName,
    }
    const startLottoGameAccounts: StartLottoGameAccounts = {
      authority: authority.publicKey,
      lollysLotto: lollysLottoPda,
      lottoGame: lottoGamePda,
      lottoGameVaultSigner: lottoGameVaultSigner,
      lottoGameVault: lottoGameVaultPda,
      lottoGameMint: USDC_MINT_DEVNET,
      eventEmitter: eventEmitterPda,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    };
    startLottoGameIx = startLottoGame(startLottoGameArgs, startLottoGameAccounts, lollysLottoProgram.programId);

    // const sig = await sendVersionedTx(connection, [startLottoGameIx], authority.publicKey, [authority]);
    // console.log(`[TX] startLottoGame: ${sig}`);

    // const startLottoGameSig = await lollysLottoProgram.methods
    //   .startLottoGame(round, ticketPrice, gameDuration, randomnessAccount, roundName)
    //   .accounts({
    //     authority: authority.publicKey,
    //     lollysLotto: lollysLottoPda,
    //     lottoGame: lottoGamePda,
    //     lottoGameVaultSigner: lottoGameVaultSigner,
    //     lottoGameVault: lottoGameVault,
    //     lottoGameMint: USDC_MINT_DEVNET,
    //     randomnessAccountData: randomnessAccount,
    //     eventEmitter: eventEmitterPda,
    //     tokenProgram: TOKEN_PROGRAM_ID,
    //     associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //     systemProgram: SystemProgram.programId,
    //   })
    //   .signers([authority])
    //   .rpc();
    // console.log(`[TX] startLottoGame: ${startLottoGameSig}`);
  }

  return {
    lottoGamePda,
    lottoGameVaultPda,
    lottoGameVaultSigner,
    startLottoGameIx,
  };
}