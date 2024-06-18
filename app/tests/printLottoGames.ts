import * as anchor from "@coral-xyz/anchor";
import { Connection } from "@solana/web3.js";
import { getLottoGamePda, loadWalletKey } from "../src/utils";
import fs from "fs";
import path from "path";
import { LottoGame } from "../../lollys-lotto-ts-sdk/src/codegen/accounts";

async function fetchLottoGameAccount(connection: Connection, program: anchor.Program, pda: anchor.web3.PublicKey, round: anchor.BN) {
  try {
    // const account = await LottoGame.fetch(connection, pda);
    const account = await program.account.lottoGame.fetch(pda);
    const directoryPath = path.join(__dirname, './testLottoGameAccounts');
    const filePath = path.join(directoryPath, `lotto_game_round_${round}.json`);

    // Create the directory if it doesn't exist
    if (!fs.existsSync(directoryPath)) {
      fs.mkdirSync(directoryPath, { recursive: true });
    }

    // fs.writeFileSync(filePath, account);
    console.log(`Lotto Game Account for ${round}: ${account}`);
    console.log(`LottoGame.bump: ${account.bump}`);
    console.log(`LottoGame.lottoGameVaultBump: ${account.lottoGameVaultBump}`);
    console.log(`LottoGame.version: ${account.version}`);
    console.log(`LottoGame.state: ${account.state}`);
    console.log(`LottoGame.authority: ${account.authority}`);
    console.log(`LottoGame.round: ${account.round}`);
    console.log(`LottoGame.startDate: ${account.startDate}`);
    console.log(`LottoGame.endDate: ${account.endDate}`);
    console.log(`LottoGame.ticketPrice: ${account.ticketPrice}`);
    console.log(`LottoGame.ticketsSold: ${account.ticketsSold}`);
    console.log(`LottoGame.lottoGameMint: ${account.lottoGameMint}`);
    
    
    // fs.writeFileSync(filePath, JSON.stringify(account.toJSON(), null, 2));
    // console.log(`Lotto Game Account for Round ${round} written to ${filePath}`);
    return account;
  } catch (error) {
    console.error(`Error fetching Lotto Game Account for Round ${round}:`, error.message);
    return null;
  }
}

async function main() {
  try {
    const connection = new Connection("https://omniscient-frequent-wish.solana-devnet.quiknode.pro/94f987a0b2e2936be10a5f9d3eb81058c60681b9/", "confirmed");
    const authority = loadWalletKey("/Users/0xabstracted/Lolly/lollys-lotto/.keys/lollys_lotto_authority.json");
    console.log(`Authority: ${authority.publicKey.toBase58()}`);
    const authorityWallet = new anchor.Wallet(authority);
    const provider = new anchor.AnchorProvider(connection, authorityWallet, {});

    const lollysLottoProgramKeypair = loadWalletKey("/Users/0xabstracted/Lolly/lollys-lotto/target/deploy/lollys_lotto-keypair.json");
    const lollysLottoProgramId = lollysLottoProgramKeypair.publicKey;
    const lollysLottoIdl = (await anchor.Program.fetchIdl(lollysLottoProgramId, provider))!;
    const lollysLottoProgram = new anchor.Program(lollysLottoIdl, lollysLottoProgramId, provider);

    const rounds = [0, 1, 2, 3, 4, 5].map(round => new anchor.BN(round));

    // Step 1: Get PDAs
    let pdaResults = [];
    for (let i = 0; i < rounds.length; i++) {
      const pda = getLottoGamePda(authority.publicKey, rounds[i], lollysLottoProgramId);
      pdaResults.push(pda);
    }

    // Step 2: Fetch LottoGame accounts
    let fetchResults = [];
    for (let i = 0; i < pdaResults.length; i++) {
      const fetchResult = await fetchLottoGameAccount(connection, lollysLottoProgram, pdaResults[i], rounds[i]);
      fetchResults.push(fetchResult);
    }

  } catch (error) {
    console.error("Error in main process:", error.message);
  }
}

main().catch(console.error);
