import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import { 
    BuyLottoTicketAccounts, 
    BuyLottoTicketArgs, 
    buyLottoTicket,
} from "../../../lollys-lotto-ts-sdk/src/codegen/instructions";
import { 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  TOKEN_PROGRAM_ID, 
  getAssociatedTokenAddress 
} from "@solana/spl-token";
import { 
  getLottoGameVaultPda, 
  getLottoGameVaultSignerPda, 
  getLottoTicketPda, 
  sendVersionedTx 
} from "../utils";
import { LottoTicketNumbersFields } from "../../../lollys-lotto-ts-sdk/src/codegen/types";
import { USDC_MINT_DEVNET } from "../constants";

export async function buyLottoTicketTx(
    lollysLottoProgram: anchor.Program,
    authority: PublicKey,
    user: Keypair,
    userMetadataPda: PublicKey,
    round: anchor.BN,
    lottoNumbers: LottoTicketNumbersFields,
    lottoGamePda: PublicKey,
    eventEmitterPda: PublicKey,
    connection: Connection,
): Promise<PublicKey> {
    const lottoGameVaultSigner = getLottoGameVaultSignerPda(lottoGamePda, lollysLottoProgram.programId);
    const lottoGameVaultPda = await getLottoGameVaultPda(lottoGameVaultSigner);
    const lottoTicketPda = getLottoTicketPda(lottoGamePda, userMetadataPda, lottoNumbers, lollysLottoProgram.programId);
    const userUsdcTokenAccount = await getAssociatedTokenAddress(USDC_MINT_DEVNET, user.publicKey, false);

    try {
      const lottoTicketAccount = await lollysLottoProgram.account.lottoTicket.fetch(lottoTicketPda);
      console.log(`Lotto Ticket TicketNumber ${JSON.stringify(lottoTicketAccount.ticketNumber)}`);
    } catch {
      const buyLottoTicketArgs: BuyLottoTicketArgs = {
          round,
          lottoTicketNumbers: lottoNumbers
      }

      const buyLottoTicketAccounts: BuyLottoTicketAccounts = {
          authority,
          user: user.publicKey,
          userMetadata: userMetadataPda,
          userUsdcTokenAccount,
          lottoGameMint: USDC_MINT_DEVNET,
          lottoGame: lottoGamePda,
          lottoGameVault: lottoGameVaultPda,
          lottoTicket: lottoTicketPda,
          eventEmitter: eventEmitterPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
      };

      const buyLottoTicketIx = buyLottoTicket(buyLottoTicketArgs, buyLottoTicketAccounts, lollysLottoProgram.programId);
      const sig = await sendVersionedTx(connection, [buyLottoTicketIx], user.publicKey, [user]);
      console.log(`[TX] buyLottoTicket: ${sig}`);
    }

    return lottoTicketPda;
}
