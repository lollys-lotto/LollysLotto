import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  AnchorUtils,
  Queue,
  Randomness,
  SB_ON_DEMAND_PID,
  sleep,
} from "@switchboard-xyz/on-demand";
import { 
  loadWalletKey, 
  sendVersionedTx 
} from "./utils";
import { createEventEmitterIfNotExist } from "./admin/createEventEmitter";
import { createLollysLottoIfNotExist } from "./admin/createLolysLotto";
import { startLottoGameIxIfNotExist } from "./admin/startLottoGame";
import { createUserMetadataIfNotExist } from "./user/createUserMetadata";
import { LottoTicketNumbersFields } from "../../lollys-lotto-ts-sdk/src/codegen/types";
import { buyLottoTicketTx } from "./user/buyTicket";
import { getProcessWinningNumbersIx } from "./switchboard/processWinningNumbers";
import { crankLottoGameClosedTx } from "./cranks/crankLottoGameClosed";
import { crankLottoGameWinnersTx } from "./cranks/crankLottoGameWinners";
import { crankTransferWinningAmountToUserRewardsVaultTx } from "./cranks/crankTransferWinningAmountToUserRewardsVault";
import { claimUserRewardsTx } from "./user/claimUserRewards";

async function main() {
    try {
      const connection = new Connection("https://omniscient-frequent-wish.solana-devnet.quiknode.pro/94f987a0b2e2936be10a5f9d3eb81058c60681b9/", "confirmed");
      
      const authority = loadWalletKey("/Users/0xabstracted/Lolly/lollys-lotto/.keys/lollys_lotto_authority.json");
      console.log(`Authority: ${authority.publicKey.toBase58()}`);
      const authorityWallet = new anchor.Wallet(authority);
  
      let provider = new anchor.AnchorProvider(connection, authorityWallet, {});
  
      // Switchboard sbQueue fixed
      const sbQueue = new PublicKey("FfD96yeXs4cxZshoPPSKhSPgVQxLAJUT3gefgh84m1Di");
      const sbProgramId = SB_ON_DEMAND_PID;
      const sbIdl = await anchor.Program.fetchIdl(sbProgramId, provider);
      const sbProgram = new anchor.Program(sbIdl, sbProgramId, provider);
      const queueAccount = new Queue(sbProgram, sbQueue);
  
      // setup
      const path = "/Users/0xabstracted/Lolly/lollys-lotto/target/deploy/lollys_lotto-keypair.json";
      const [_, myProgramKeypair] = await AnchorUtils.initWalletFromFile(path);
      const lollysLottoProgramId = myProgramKeypair.publicKey;
      const lollysLottoIdl = (await anchor.Program.fetchIdl(lollysLottoProgramId, provider))!;
      const lollysLottoProgram = new anchor.Program(lollysLottoIdl, lollysLottoProgramId, provider);
  
      // 0. Create a Randomness Account using Switchboard
      const randomnessPath = "/Users/0xabstracted/Lolly/lollys-lotto/.keys/randomness_account.json";
      const randomessKeypair = await AnchorUtils.initKeypairFromFile(randomnessPath);
      const [randomness, ix] = await Randomness.create(sbProgram, randomessKeypair, sbQueue);
  
      /// Uncomment the below snippet to create randomness account
      // const tx = await InstructionUtils.asV0Tx(sbProgram, [ix]);
      // console.log("Sending Randomness account creation Tx.. ");
      // tx.sign([authority, randomessKeypair]);
      // const sig1 = await connection.sendTransaction(tx);
      // await connection.confirmTransaction(sig1);
  
      const randomnessAccount = randomness.pubkey;
      console.log("Randomness Account: ", randomnessAccount.toBase58());
  
      
      // 1. Create Event Emitter Account with Admin
      const eventEmitterPda =  await createEventEmitterIfNotExist(lollysLottoProgram, authority, connection);
      // 2. Create Lollys Lotto Account
      const lollysLottoPda = await createLollysLottoIfNotExist(lollysLottoProgram, authority, eventEmitterPda);
      // 3. Start Lotto Game  Round 1
      const round1 = new anchor.BN(3);
      const ticketPrice: anchor.BN = new anchor.BN(1000000);
      // const gameDuration: anchor.BN = new anchor.BN(1200); //20 minutes
      const gameDuration: anchor.BN = new anchor.BN(120); //2 minutes
      const roundName = "Lolly's Lotto Round 4";
      console.log(`Round Name: ${roundName}`);
      console.log("Buffer.from('lotto-game'):", Buffer.from('lotto-game'));
      console.log("authority.publicKey.toBuffer():", authority.publicKey.toBuffer());
      console.log("Buffer.from(roundName):", Buffer.from(roundName));
      const { lottoGamePda, lottoGameVaultPda, lottoGameVaultSigner, startLottoGameIx } = await startLottoGameIxIfNotExist(
        lollysLottoProgram,
        authority,
        round1,
        ticketPrice,
        gameDuration,
        roundName,
        randomnessAccount,
        lollysLottoPda,
        eventEmitterPda,
        connection,
      );
  
      // 4. Transaction to commit to the oracle's prediction. Formally ask for the outcome based on future slot. 
      let commitIx: TransactionInstruction | null = null;
      try {
        commitIx = await randomness.commitIx(sbQueue);
      } catch (error) {
        try {
          await queueAccount.fetchFreshOracle();
        } catch (error) {
          // const tx = await InstructionUtils.asV0Tx(sbProgram, [ix]);
          // console.log("");
          // tx.sign([payer, randomessKeypair]);
          // const sig1 = await connection.sendTransaction(tx);
          // await connection.confirmTransaction(sig1);
          console.error(
            "Failed to find an open oracle. Please check our docs to ensure queue ${sbQueue} is an active queue."
          );
          // throw error;
        }
        throw error;
      }
      if (commitIx && startLottoGameIx) {
        const sig = await sendVersionedTx(connection, [commitIx, startLottoGameIx], authority.publicKey, [authority]);
        console.log(`[TX] commitIx and startLottoGame: ${sig}`);
      } else if (startLottoGameIx) {
        const sig = await sendVersionedTx(connection, [startLottoGameIx], authority.publicKey, [authority]);
        console.log(`[TX] startLottoGame: ${sig}`);
      }

      /// Users addresses read from the keys
      const user1 = loadWalletKey("/Users/0xabstracted/Lolly/lollys-lotto/.keys/user1.json");
      console.log(`User1: ${user1.publicKey.toBase58()}`);
      const user1Wallet = new anchor.Wallet(user1);

      const user2 = loadWalletKey("/Users/0xabstracted/Lolly/lollys-lotto/.keys/user2.json");
      console.log(`User2: ${user2.publicKey.toBase58()}`);
      const user2Wallet = new anchor.Wallet(user2);

      provider = new anchor.AnchorProvider(connection, user1Wallet, {});

      // 5. Create User Metadata and Rewards Vault for User1 and User2
      const {
        userMetadataPda: userMetadataPda1, 
        userRewardsVault: userRewardsVault1
      } = await createUserMetadataIfNotExist(lollysLottoProgram, user1, eventEmitterPda, connection);
      const {
        userMetadataPda: userMetadataPda2, 
        userRewardsVault: userRewardsVault2
      } = await createUserMetadataIfNotExist(lollysLottoProgram, user2, eventEmitterPda, connection);
  
      const lottoNumbers1: LottoTicketNumbersFields = {
          number1: 0,
          number2: 0,
          number3: 0,
          number4: 0,
          number5: 0,
          jackpotNumber: 0,
      };
  
      const lottoNumbers2: LottoTicketNumbersFields = {
        number1: 0,
        number2: 0,
        number3: 0,
        number4: 0,
        number5: 0,
        jackpotNumber: 1,
      };
  
      const lottoNumbers3: LottoTicketNumbersFields = {
        number1: 0,
        number2: 0,
        number3: 0,
        number4: 0,
        number5: 0,
        jackpotNumber: 2,
      };
  
      const lottoNumbers4: LottoTicketNumbersFields = {
        number1: 0,
        number2: 0,
        number3: 0,
        number4: 0,
        number5: 0,
        jackpotNumber: 3,
      };
  
      const lottoNumbers5: LottoTicketNumbersFields = {
        number1: 0,
        number2: 0,
        number3: 0,
        number4: 0,
        number5: 0,
        jackpotNumber: 4,
      };
  
      // 6. Buy Lotto Tickets
      // buy lottoNumbers1 and lottoNumbers3, lottoNumbers5 for user1
      let lottoTicketPda1 = await buyLottoTicketTx(
        lollysLottoProgram, 
        authority.publicKey, 
        user1,
        userMetadataPda1, 
        round1, 
        lottoNumbers1, 
        lottoGamePda, 
        eventEmitterPda, 
        connection
      );
      console.log(`1. Lotto Ticket PDA 1: ${lottoTicketPda1.toBase58()}`);
  
      lottoTicketPda1 = await buyLottoTicketTx(
        lollysLottoProgram, 
        authority.publicKey, 
        user1,
        userMetadataPda1, 
        round1, 
        lottoNumbers3, 
        lottoGamePda, 
        eventEmitterPda, 
        connection
      );
      console.log(`2. Lotto Ticket PDA 1: ${lottoTicketPda1.toBase58()}`);
  
      lottoTicketPda1 = await buyLottoTicketTx(
        lollysLottoProgram, 
        authority.publicKey, 
        user1,
        userMetadataPda1, 
        round1, 
        lottoNumbers5, 
        lottoGamePda, 
        eventEmitterPda, 
        connection
      );
      console.log(`3. Lotto Ticket PDA 1: ${lottoTicketPda1.toBase58()}`);
  
      // buy lottoNumbers2 and lottoNumbers4 for user2
      let lottoTicketPda2 = await buyLottoTicketTx(
        lollysLottoProgram, 
        authority.publicKey, 
        user2,
        userMetadataPda2, 
        round1, 
        lottoNumbers2, 
        lottoGamePda, 
        eventEmitterPda, 
        connection
      );
      console.log(`1. Lotto Ticket PDA 2: ${lottoTicketPda2.toBase58()}`);
  
      lottoTicketPda2 = await buyLottoTicketTx(
        lollysLottoProgram, 
        authority.publicKey, 
        user2,
        userMetadataPda2, 
        round1, 
        lottoNumbers4, 
        lottoGamePda, 
        eventEmitterPda, 
        connection
      );
    console.log(`2. Lotto Ticket PDA 2: ${lottoTicketPda2.toBase58()}`);
    console.log(`Lotto Ticket PDA 1: ${lottoTicketPda1.toBase58()}`);
    console.log(`Lotto Ticket PDA 2: ${lottoTicketPda2.toBase58()}`);  

    // 7. Close Round1

    const crankLottoGameCloseSig = await crankLottoGameClosedTx(
      lollysLottoProgram,
      authority,
      round1,
      lottoGamePda,
      eventEmitterPda,
      connection,
    );
    console.log("crankLottoGameCloseSig: ", crankLottoGameCloseSig);
    
    // 8. Reveal the winning numbers from the randomness account and process the winning numbers for all tiers
    const revealAndProcessWinningNumbersTx = new Transaction();
    let revealIx = undefined;
    const processWinningNumbersIx = getProcessWinningNumbersIx(
      lollysLottoProgram,
      authority.publicKey,
      randomnessAccount,
      lottoGamePda,
      eventEmitterPda,
      connection
    );

    const tries = 5;
    for (let i = 0; i < tries; ++i) {
    try {
          revealIx = await randomness.revealIx();
          randomness.serializeIxToFile(
          [revealIx, processWinningNumbersIx],
          "serializedIx.bin"
          );
          break;
      } catch (error) {
          if (i === tries - 1) {
          throw error;
          }
          console.log(
          "Waiting for a tiny bit more for the commitment to be locked..."
          );
          await sleep(1000);
      }
      } 
      // Add the reveal instruction and processWinningNumbersIx to the transaction
      revealAndProcessWinningNumbersTx.add(revealIx!, processWinningNumbersIx);
      const revealAndProcessWinningNumbersSig = await provider.sendAndConfirm(
        revealAndProcessWinningNumbersTx, 
        [authority], 
        { commitment: "confirmed"}
      );
      console.log(`revealIx and processWinningNumbersIx Transaction Signature: ${revealAndProcessWinningNumbersSig}`);
    
      // 9. Crank Lotto Game Winning Numbers for Round 1

      const crankLottoGameWinnerSig = await crankLottoGameWinnersTx(
        lollysLottoProgram,
        authority,
        user1.publicKey,
        userMetadataPda1,
        round1,
        lottoNumbers1,
        lottoGamePda,
        eventEmitterPda,
        connection
      );

      console.log("crankLottoGameWinnerSig: ", crankLottoGameWinnerSig);

      const winningNumbers: LottoTicketNumbersFields = {
        number1: 0,
        number2: 0,
        number3: 0,
        number4: 0,
        number5: 0,
        jackpotNumber: 0,
      };
      const numberOfTicketsWithDuplicateNumbers: number = 2;
      // 10. Crank Transfer Winning Amount to User Rewards Vault for all winners
      const crankTransferWinningAmountToUserRewardsVaultSig = await crankTransferWinningAmountToUserRewardsVaultTx(
        lollysLottoProgram,
        round1,
        winningNumbers,
        numberOfTicketsWithDuplicateNumbers,
        authority,
        lottoGamePda,
        user1.publicKey,
        userMetadataPda1,
        lottoNumbers1,
        eventEmitterPda,
        connection
      );
      console.log("crankTransferWinningAmountToUserRewardsVaultSig: ", crankTransferWinningAmountToUserRewardsVaultSig);

      // 11.  Claim winning amount for Jackpot winner
      const claimUserRewardsSig = await claimUserRewardsTx(
        lollysLottoProgram,
        new anchor.BN(1000),
        user1,
        eventEmitterPda,
        connection
      );
      console.log("claimUserRewardsSig: ", claimUserRewardsSig);


    } catch (error) {
      console.error("Error in main process:", error);
  }


}
main().catch(console.error);
