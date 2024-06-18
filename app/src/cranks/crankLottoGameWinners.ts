import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection,
  PublicKey,
  Keypair,
} from "@solana/web3.js";
import { LottoTicketNumbersFields } from "../../../lollys-lotto-ts-sdk/src/codegen/types";
import { 
    getLottoGameVaultPda, 
    getLottoGameVaultSignerPda, 
    getLottoTicketPda, 
    sendVersionedTx 
} from "../utils";
import { 
    CrankLottoGameWinnersAccounts, 
    crankLottoGameWinners,
    CrankLottoGameWinnersArgs 
} from "../../../lollys-lotto-ts-sdk/src/codegen/instructions";

export async function crankLottoGameWinnersTx(
    lollysLottoProgram: anchor.Program,
    authority: Keypair,
    user: PublicKey,
    userMetadataPda: PublicKey,
    round: anchor.BN,
    lottoNumbers: LottoTicketNumbersFields,
    lottoGamePda: PublicKey,
    eventEmitterPda: PublicKey,
    connection: Connection,
): Promise<string> {
    const lottoGameVaultSigner = getLottoGameVaultSignerPda(lottoGamePda, lollysLottoProgram.programId);
    const lottoGameVaultPda = await getLottoGameVaultPda(lottoGameVaultSigner);
    const lottoTicketPda = getLottoTicketPda(lottoGamePda, userMetadataPda, lottoNumbers, lollysLottoProgram.programId);
    const winningNumbers: LottoTicketNumbersFields = {
        number1: 1,
        number2: 2,
        number3: 3,
        number4: 4,
        number5: 5,
        jackpotNumber: 6,
    };

    const winningNumbersIndex: Array<anchor.BN> = [new anchor.BN(0), new anchor.BN(-1), new anchor.BN(-1), new anchor.BN(-1)];
    let sig: string;
    try {
        const crankLottoGameWinnersArgs: CrankLottoGameWinnersArgs = {
            round,
            winningNumbers,
            winningNumbersIndex,
        };
        const crankLottoGameWinnersAccounts: CrankLottoGameWinnersAccounts = {
            authority: authority.publicKey,
            lottoGame: lottoGamePda,
            lottoGameVaultSigner,
            lottoGameVault: lottoGameVaultPda,
            user: user,
            userMetadata: userMetadataPda,
            lottoTicket: lottoTicketPda,
            eventEmitter: eventEmitterPda,
            tokenProgram: TOKEN_PROGRAM_ID,
        };
        const crankLottoGameWinnersIx = crankLottoGameWinners(crankLottoGameWinnersArgs, crankLottoGameWinnersAccounts, lollysLottoProgram.programId);
        sig = await sendVersionedTx(connection, [crankLottoGameWinnersIx], authority.publicKey, [authority]);
        console.log(`[TX] crankLottoGameWinner: ${sig}`);
    } catch {
        console.log(`Error cranking Lotto Game Winners for round ${round.toNumber()}`);
    }
    return sig;
}
