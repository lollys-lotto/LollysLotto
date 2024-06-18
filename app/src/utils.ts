import * as anchor from "@coral-xyz/anchor";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import {
  Connection,
  PublicKey,
  Keypair,
  VersionedTransaction,
  TransactionInstruction,
  TransactionMessage,
  SendOptions,
  Signer,
} from "@solana/web3.js";
import * as fs from "fs";
import { LottoTicketNumbersFields } from "../../lollys-lotto-ts-sdk/src/codegen/types";
import { USDC_MINT_DEVNET } from "./constants";


export function getLollysLottoPda(authority: PublicKey, lollysLottoProgramId: PublicKey): PublicKey {
    const lollysLottoSeeds = [Buffer.from("lollys-lotto"), authority.toBuffer()];
    const [lollysLottoPda, _lollysLottoBump] = PublicKey.findProgramAddressSync(
        lollysLottoSeeds,
        lollysLottoProgramId,
    );
    console.log(`Lollys Lotto PDA: ${lollysLottoPda.toBase58()}`);
    return lollysLottoPda;
}

export function getLottoGamePda(authority: PublicKey, round: anchor.BN, lollysLottoProgramId: PublicKey): PublicKey {
    const lottoGameSeeds = [Buffer.from("lotto-game"), authority.toBuffer(), round.toArrayLike(Buffer, "le", 8)];
    const [lottoGamePda, _lottoGameBump] = PublicKey.findProgramAddressSync(lottoGameSeeds, lollysLottoProgramId);
    console.log(`Lotto Game PDA: ${lottoGamePda.toBase58()} for round ${round.toNumber()}`);
    return lottoGamePda;
}

export function getLottoGameVaultSignerPda(lottoGamePda: PublicKey, lollysLottoProgramId: PublicKey): PublicKey {
    const lottoGameVaultSignerSeeds = [Buffer.from("lotto-game-vault"), lottoGamePda.toBuffer()];
    const [lottoGameVaultSigner, _lottoGameVaultSignerBump] = PublicKey.findProgramAddressSync(
        lottoGameVaultSignerSeeds,
        lollysLottoProgramId
    );
    console.log(`Lotto Game Vault PDA: ${lottoGameVaultSigner.toBase58()}`);
    return lottoGameVaultSigner;
}

export async function getLottoGameVaultPda(lottoGameVaultSigner: PublicKey): Promise<PublicKey> {
    const lottoGameVault = await getAssociatedTokenAddress(USDC_MINT_DEVNET, lottoGameVaultSigner, true);
    console.log(`Lotto Game Vault: ${lottoGameVault.toBase58()}`);
    return lottoGameVault;
}

export function getUserMetadataPda(user: PublicKey,lollysLottoProgramId: PublicKey): PublicKey {
    const userMetadataSeeds = [Buffer.from("user-metadata"), user.toBuffer()];
    const [userMetadataPda, _userMetadataBump] = PublicKey.findProgramAddressSync(userMetadataSeeds, lollysLottoProgramId);
    console.log(`User Metadata PDA: ${userMetadataPda.toBase58()}`);
    return userMetadataPda;
}

export async function getUserRewardsVaultPda(user: PublicKey, lollysLottoProgramId: PublicKey): Promise<PublicKey> {
    const userMetadataPda = getUserMetadataPda(user, lollysLottoProgramId);
    const userRewardsVaultPda = await getAssociatedTokenAddress(USDC_MINT_DEVNET, userMetadataPda, true);
    console.log(`User Rewards Vault PDA: ${userRewardsVaultPda.toBase58()}`);
    return userRewardsVaultPda;

}

export function getLottoTicketPda(lottoGame: PublicKey, userMetadata: PublicKey, lottoTicketNumbers: LottoTicketNumbersFields, lollysLottoProgramId: PublicKey): PublicKey {
    const lottoTicketSeeds = [
        Buffer.from("lotto-ticket"), 
        lottoGame.toBuffer(), 
        userMetadata.toBuffer(), 
        Buffer.from([lottoTicketNumbers.number1]), 
        Buffer.from([lottoTicketNumbers.number2]),
        Buffer.from([lottoTicketNumbers.number3]),
        Buffer.from([lottoTicketNumbers.number4]),
        Buffer.from([lottoTicketNumbers.number5]),
        Buffer.from([lottoTicketNumbers.jackpotNumber]),
    ];
    const [lottoTicketPda, lottoTicketBump] = PublicKey.findProgramAddressSync(lottoTicketSeeds, lollysLottoProgramId);
    console.log(`Lotto Ticket PDA: ${lottoTicketPda.toBase58()}`);
    return lottoTicketPda;
}

export function getEventEmitterPda(lollysLottoProgramId: PublicKey): PublicKey {
    const eventEmitterSeeds = [Buffer.from("event-emitter")];
    const [eventEmitterPda, _eventEmitterBump] = PublicKey.findProgramAddressSync(
        eventEmitterSeeds,
        lollysLottoProgramId
    );

    console.log("Event Emitter PDA: ", eventEmitterPda.toBase58());

    return eventEmitterPda;

}

export function loadWalletKey(keypairFile:string): Keypair {
    if (!keypairFile || keypairFile == '') {
      throw new Error('Keypair is required!');
    }
    const loaded = Keypair.fromSecretKey(
      new Uint8Array(JSON.parse(fs.readFileSync(keypairFile).toString())),
    );
    return loaded;
  }
  
export async function sendVersionedTx(
    connection: Connection, 
    instructions: TransactionInstruction[], 
    payer: PublicKey,
    signers: Signer[]): Promise<string> {
    let latestBlockhash = await connection.getLatestBlockhash()
    const messageLegacy = new TransactionMessage({
        payerKey: payer,
        recentBlockhash: latestBlockhash.blockhash,
        instructions,
    }).compileToLegacyMessage();
    const transation = new VersionedTransaction(messageLegacy)
    transation.sign(signers);
    const options: SendOptions = {
      maxRetries: 3,
      skipPreflight: false,
    };
    const signature = await connection.sendTransaction(transation, options);
    return signature;
  }
  