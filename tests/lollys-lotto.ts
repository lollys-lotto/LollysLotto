import type { LollysLotto } from "../target/types/lollys_lotto";

import * as anchor from "@coral-xyz/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { RandomnessService } from "@switchboard-xyz/solana-randomness-service";
import assert from "assert";
import { PublicKey, Connection, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { loadKeypair } from "@switchboard-xyz/solana.js";
import BN from "bn.js";
describe("lollys-lotto", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.LollysLotto as anchor.Program<LollysLotto>;

  function stringToByteArray(str) {
    // Create a TextEncoder instance
    const encoder = new TextEncoder();

    // Encode the string into a Uint8Array containing UTF-8 encoded text
    const byteArray = encoder.encode(str);

    // Return the byte array
    return byteArray;
  }

  function stringToU8Array(str) {
    // Create a TextEncoder instance
    const encoder = new TextEncoder();

    // Encode the string into a Uint8Array containing UTF-8 encoded text
    const uint8Array = encoder.encode(str);

    // Convert the Uint8Array to a regular array of numbers
    const numArray = Array.from(uint8Array);

    // Return the array of numbers
    return numArray;
}

  let randomnessService: RandomnessService;

  before(async () => {
    randomnessService = await RandomnessService.fromProvider(provider);
  });


  it("requests randomness", async () => {
    // Add your test here.
    const requestKeypair = anchor.web3.Keypair.generate();
    console.log(`Request: ${requestKeypair.publicKey.toBase58()}`);

    const authority = loadKeypair("/Users/0xabstracted/Lolly/LollyLottoTS/.keys/1o1ohFR7M25XktNXAsbnDbvserNoFkrFLdA9916EGWw.json");
    console.log(`Authority: ${authority.publicKey.toBase58()}`);
    
    // Start watching for the settled event before triggering the request
    const settledRandomnessEventPromise = randomnessService.awaitSettledEvent(
      requestKeypair.publicKey
    );

    const eventEmitterSeeds = [Buffer.from('event-emitter')];
    const [eventEmitterPda, eventEmitterBump] = await PublicKey.findProgramAddressSync(eventEmitterSeeds, program.programId);
    console.log(`Event Emitter PDA: ${eventEmitterPda.toBase58()}`);

    try {
      const eventEmitterAccount = await program.account.eventEmitter.fetch(eventEmitterPda);
      console.log(`Event Emitter Account: ${JSON.stringify(eventEmitterAccount)}`);
    } catch {
      const createEventEmitterSig = await program.methods
        .createEventEmitter()
        .accounts({
          funder: authority.publicKey,
          eventEmitter: eventEmitterPda,
        })
        .signers([authority])
        .rpc();
      console.log(`[TX] createEventEmitter: ${createEventEmitterSig}`);
    }
    
    
    const lollysLottoSeeds = [Buffer.from('lollys-lotto'), authority.publicKey.toBuffer()];
    const [lollysLottoPda, lollysLottoBump] = await PublicKey.findProgramAddressSync(lollysLottoSeeds, program.programId);
    console.log(`Lolly Lotto PDA: ${lollysLottoPda.toBase58()}`);
    let lollysLottoAccount = null;
    try {
      lollysLottoAccount = await program.account.lollysLotto.fetch(lollysLottoPda);
      console.log(`Lolly Lotto Account: ${JSON.stringify(lollysLottoAccount)}`);
      console.log(`lollysLottoAccount.lottoGameCount: ${lollysLottoAccount.lottoGameCount}`);
    } catch {
      const createLollysLottoSig = await program.methods
        .createLollysLotto()
        .accounts({
          authority: authority.publicKey,
          lollysLotto: lollysLottoPda,
          eventEmitter: eventEmitterPda,
        })
        .signers([authority])
        .rpc();
      console.log(`[TX] createLollysLotto: ${createLollysLottoSig}`);
    }

    const USDC_MINT_DEVNET = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
    const programId = program.programId;
    console.log(`Program ID: ${programId.toBase58()}`);
    
    
    // const roundNameByteArray = stringToByteArray(roundNameString);
    // console.log(`Round Name Byte Array: ${roundNameByteArray}`);
    // let roundName = stringToU8Array(roundNameString);
    // if (roundName.length > 32) {
    //   roundName = roundName.slice(0, 32);
    // } else while (roundName.length < 32) {
    //   roundName.push(0);
    // }

    // Remove trailing zeros
    // while (roundName.length > 0 && roundName[roundName.length - 1] === 0) {
    //   roundName.pop();
    // }
    
    // console.log(`Round Name: ${roundName}`);
    // console.log("Buffer.from('lotto-game'):", Buffer.from('lotto-game'));
    // console.log("authority.publicKey.toBuffer():", authority.publicKey.toBuffer());
    // console.log("Buffer.from(roundName):", Buffer.from(roundName));
    // let lottoGameSeeds = null
    // if (lollysLottoAccount !== null) {
    //   // Seeds for generating the PDA. These can be any byte array. Often a combination of account keys and program-specific seed strings are used.
    //   lottoGameSeeds = [Buffer.from('lotto-game'), authority.publicKey.toBuffer(), roundName];
    // }
    // Find the PDA
    // const [lottoGamePda, lottoGameBump] = await PublicKey.findProgramAddressSync(lottoGameSeeds, programId);
    // console.log(`Lotto Game PDA: ${lottoGamePda.toBase58()}`);

    const lottoGameVaultSignerSeeds = [Buffer.from('lotto-game-vault')];

    const [lottoGameVaultSigner, lottoGameVaultSignerBump] = await PublicKey.findProgramAddressSync(lottoGameVaultSignerSeeds, programId);
    console.log(`Lotto Game Vault Signer: ${lottoGameVaultSigner.toBase58()}`);

    const lottoGameVault = await getAssociatedTokenAddress(USDC_MINT_DEVNET, lottoGameVaultSigner, true);
    console.log(`Lotto Game Vault: ${lottoGameVault.toBase58()}`);


    const round = new anchor.BN(0);
    const ticketPrice: anchor.BN = new anchor.BN(1000000);
    const gameDuration: anchor.BN = new anchor.BN(86400);
    const roundName = "Lolly's Lotto Round 1";
    console.log(`Round Name: ${roundName}`);
    console.log("Buffer.from('lotto-game'):", Buffer.from('lotto-game'));
    console.log("authority.publicKey.toBuffer():", authority.publicKey.toBuffer());
    console.log("Buffer.from(roundName):", Buffer.from(roundName));
    let lottoGameSeeds = null
    if (lollysLottoAccount !== null) {
      // Seeds for generating the PDA. These can be any byte array. Often a combination of account keys and program-specific seed strings are used.
      lottoGameSeeds = [Buffer.from('lotto-game'), authority.publicKey.toBuffer(), roundName];
    }
    // Find the PDA
    const [lottoGamePda, lottoGameBump] = await PublicKey.findProgramAddressSync(lottoGameSeeds, programId);
    console.log(`Lotto Game PDA: ${lottoGamePda.toBase58()}`);



    // let lottoGameKeypair = anchor.web3.Keypair.generate();
    // let lottoGamePda = lottoGameKeypair.publicKey;
    try {
      const lottoGameAccount = await program.account.lottoGame.fetch(lottoGamePda);
      console.log(`Lotto Game Account: ${JSON.stringify(lottoGameAccount)}`);
    }
    catch {
      const startLottoGameSig = await program.methods
        .startLottoGame(round, ticketPrice, gameDuration, roundName)
        .accounts({
          authority: authority.publicKey,
          lollysLotto: lollysLottoPda,
          lottoGame: lottoGamePda,
          lottoGameVaultSigner: lottoGameVaultSigner,
          lottoGameVault: lottoGameVault,
          usdcMint: USDC_MINT_DEVNET,
          eventEmitter: eventEmitterPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();
      console.log(`[TX] startLottoGame : ${startLottoGameSig}`);
    }

    // your program makes a CPI request to the RandomnessService
    const signature = await program.methods
      .requestRandomness()
      .accounts({
        randomnessService: randomnessService.programId,
        randomnessRequest: requestKeypair.publicKey,
        randomnessEscrow: anchor.utils.token.associatedAddress({
          mint: randomnessService.accounts.mint,
          owner: requestKeypair.publicKey,
        }),
        randomnessState: randomnessService.accounts.state,
        randomnessMint: randomnessService.accounts.mint,
        payer: provider.wallet.publicKey,
        authority: authority.publicKey,
        lottoGame: lottoGamePda,
        eventEmitter: eventEmitterPda,
      })
      .signers([requestKeypair])
      .rpc();
    console.log(`[TX] requestRandomness: ${signature}`);

    // Await the response from the Switchboard Service
    const [settledRandomnessEvent, settledSlot] =
      await settledRandomnessEventPromise;

    console.log(
      `[EVENT] SimpleRandomnessV1SettledEvent\n${JSON.stringify(
        {
          ...settledRandomnessEvent,

          // why is anchor.BN so annoying with hex strings?
          requestSlot: settledRandomnessEvent.requestSlot.toNumber(),
          settledSlot: settledRandomnessEvent.settledSlot.toNumber(),
          randomness: `[${new Uint8Array(settledRandomnessEvent.randomness)}]`,
        },
        undefined,
        2
      )}`
    );

    assert.equal(
      settledRandomnessEvent.user.toBase58(),
      provider.wallet.publicKey.toBase58(),
      "User should be the same as the provider wallet"
    );
    assert.equal(
      settledRandomnessEvent.request.toBase58(),
      requestKeypair.publicKey.toBase58(),
      "Request should be the same as the provided request keypair"
    );
    assert.equal(
      settledRandomnessEvent.isSuccess,
      true,
      "Request did not complete successfully"
    );

    const latency = settledRandomnessEvent.settledSlot
      .sub(settledRandomnessEvent.requestSlot)
      .toNumber();
    console.log(
      `\nRandomness: [${new Uint8Array(
        settledRandomnessEvent.randomness
      )}]\nRequest completed in ${latency} slots!\n`
    );

  });
});
