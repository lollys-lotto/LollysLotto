import * as anchor from "@coral-xyz/anchor";
import { Connection } from "@solana/web3.js";
import { getLottoGamePda, getLottoTicketPda, getUserMetadataPda, loadWalletKey } from "../src/utils";
import fs from "fs";
import path from "path";
import { LottoTicket } from "../../lollys-lotto-ts-sdk/src/codegen/accounts";
import { LottoTicketNumbersFields } from "../../lollys-lotto-ts-sdk/src/codegen/types/LottoTicketNumbers";

async function fetchLottoTicketAccount(connection: Connection, program: anchor.Program, pda: anchor.web3.PublicKey, round: anchor.BN) {
  try {
    const account = await LottoTicket.fetch(connection, pda);
    // const account = await program.account.lottoTicket.fetch(pda);
    const directoryPath = path.join(__dirname, './testLottoTicketAccounts');
    const filePath = path.join(directoryPath, `lotto_ticket_round_${round}_${account.ticketNumber}.json`);

    // Create the directory if it doesn't exist
    if (!fs.existsSync(directoryPath)) {
      fs.mkdirSync(directoryPath, { recursive: true });
    }

    // fs.writeFileSync(filePath, account);
    fs.writeFileSync(filePath, JSON.stringify(account, null, 2));
    console.log(`Lotto Ticket Account for Round ${round} written to ${filePath}`);
    return account;
  } catch (error) {
    console.error(`Error fetching Lotto Ticket Account for Round ${round}:`, error.message);
    return null;
  }
}

async function main() {
  try {
    const connection = new Connection("https://omniscient-frequent-wish.solana-devnet.quiknode.pro/94f987a0b2e2936be10a5f9d3eb81058c60681b9/", "confirmed");
    const authority = loadWalletKey("/Users/0xabstracted/Lolly/lollys-lotto/.keys/lollys_lotto_authority.json");
    console.log(`Authority: ${authority.publicKey.toBase58()}`);
    const authorityWallet = new anchor.Wallet(authority);
    const user1 = loadWalletKey("/Users/0xabstracted/Lolly/lollys-lotto/.keys/user1.json");
    console.log(`User1: ${user1.publicKey.toBase58()}`);
    const user2 = loadWalletKey("/Users/0xabstracted/Lolly/lollys-lotto/.keys/user2.json");
    console.log(`User2: ${user2.publicKey.toBase58()}`);
    const user1Wallet = new anchor.Wallet(user1);
    
    const provider = new anchor.AnchorProvider(connection, user1Wallet, {});



    const lollysLottoProgramKeypair = loadWalletKey("/Users/0xabstracted/Lolly/lollys-lotto/target/deploy/lollys_lotto-keypair.json");
    const lollysLottoProgramId = lollysLottoProgramKeypair.publicKey;
    const lollysLottoIdl = (await anchor.Program.fetchIdl(lollysLottoProgramId, provider))!;
    const lollysLottoProgram = new anchor.Program(lollysLottoIdl, lollysLottoProgramId, provider);

    const rounds = [0, 1, 2, 3, 4, 5].map(round => new anchor.BN(round));
    const lottoGamePdaPromises = rounds.map(round => getLottoGamePda(authority.publicKey, round, lollysLottoProgramId));
    const lottoGamePdas = await Promise.all(lottoGamePdaPromises);

    const userMetadata1Pda = getUserMetadataPda(user1.publicKey, lollysLottoProgramId);
    const userMetadata2Pda = getUserMetadataPda(user2.publicKey, lollysLottoProgramId);

    const lottoTicketNumbers1: LottoTicketNumbersFields = {
      number1: 0,
      number2: 0,
      number3: 0,
      number4: 0,
      number5: 0,
      jackpotNumber: 0,
    }

    // Read User1 ticket numbers for the current round from db
    const user1TicketPdaPromises = lottoGamePdas.map(pda => getLottoTicketPda(pda, userMetadata1Pda, lottoTicketNumbers1, lollysLottoProgramId));

    const user1TicketPdas = await Promise.all(user1TicketPdaPromises);
    const fetchPromises = user1TicketPdas.map((pda, index) => fetchLottoTicketAccount(connection, lollysLottoProgram, pda, rounds[index]));

    const user1LottoTicketAccounts = await Promise.all(fetchPromises);
    console.log("User1 Lotto Ticket Accounts:", user1LottoTicketAccounts);

  } catch (error) {
    console.error("Error in main process:", error.message);
  }
}

main().catch(console.error);
